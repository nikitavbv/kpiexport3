#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate custom_error;

use rozklad::group_schedule_by_name;
use actix_web::{App, HttpServer, Responder, get, HttpResponse, web };
use config::bind_address;
use prometheus::{TextEncoder, Encoder, Counter, register_counter, opts};
use database::database_connection;
use errors::PersistenceError;
use models::schedule::{GroupScheduleSource, GroupSchedule, GroupScheduleEntry, ScheduleWeek, ScheduleDay};
use std::env;
use crate::models::groups::{total_groups_saved, add_group};
use crate::jobs::refresh_groups::refresh_groups;
use crate::jobs::refresh_schedule::refresh_schedule;
use crate::models::schedule_queries::{remove_old_schedule_from_database, save_schedule_to_database};
use crate::rozklad_parser::Term;

mod config;
mod database;
mod e2e;
mod errors;
mod models;
mod rozklad;
mod rozklad_parser;
mod rozklad_api;
mod utils;
mod jobs;

lazy_static! {
    static ref GROUPS_LIST_REQUESTS: Counter = register_counter!(opts!(
        "kpiexport_requests_groups",
        "Total group list requests"
    )).unwrap();
    static ref GROUP_SCHEDULE_REQUESTS: Counter = register_counter!(opts!(
        "kpiexport_requests_group_schedule",
        "Total group schedule requests"
    )).unwrap();
}

#[derive(Deserialize)]
struct GroupName {
    group_name: String,
}

#[derive(Deserialize)]
struct SubjectName {
    subject_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args: Vec<String> = env::vars().map(|v| v.0).collect();
    let contains_refresh_groups = args.contains(&"KPIEXPORT_REFRESH_GROUPS_JOB".to_string());
    let contains_refresh_schedule = args.contains(&"KPIEXPORT_REFRESH_SCHEDULE_JOB".to_string());

    if contains_refresh_groups {
        println!("starting refresh groups job");
        refresh_groups().await
    } else if contains_refresh_schedule {
        println!("starting refresh schedule job");
        refresh_schedule().await
    } else {
        info!("starting kpiexport webserver");
        start_webserver().await
    }
}

async fn start_webserver() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(healthz)
        .service(metrics)
        .service(groups)
        .service(group_schedule)
        .service(subject_id_by_name)
    )
        .bind(bind_address())?
        .run()
        .await
}

#[get("/groups/{group_name}")]
async fn group_schedule(group_name: web::Path<GroupName>) -> impl Responder {
    info!("group schedule request");

    GROUP_SCHEDULE_REQUESTS.inc();

    let client = reqwest::Client::new();
    let database = match database_connection().await {
        Ok(v) => v,
        Err(err) => {
            error!("failed to connect to database: {}", err);
            return HttpResponse::InternalServerError().body("internal server error");
        }
    };

    let schedule_from_database = load_group_schedule_from_database(&database, &group_name.group_name).await
        .ok()
        .and_then(|v| v);

    let schedule = match schedule_from_database {
        Some(v) => {
            info!("from cache: {}", group_name.group_name);
            v
        },
        None => {
            info!("loading: {}", group_name.group_name);

            if let Err(err) = remove_old_schedule_from_database(&database, &group_name.group_name).await {
                error!("failed to remove schedule from database: {}", err);
            }
            info!("removed old schedule from database if present");
            
            let schedule = match group_schedule_by_name(&client, &Term::current(), &group_name.group_name).await {
                Ok(v) => v,
                Err(err) => {
                    error!("failed to get group schedule: {}", err);
                    return HttpResponse::InternalServerError().body("failed to get group schedule");
                }
            };

            if let Err(err) = save_schedule_to_database(&database, &group_name.group_name, &schedule).await {
                error!("failed to save schedule to database: {}", err);
            }

            schedule
        }
    };

    let schedule = GroupSchedule {
        entries: schedule.entries.iter().map(|v| GroupScheduleEntry {
            week: v.week.clone(),
            day: v.day.clone(),
            index: v.index.clone(),
            names: v.names.clone(),
            lecturers: v.lecturers.clone(),
            locations: v.locations.iter().map(|v| format!("НТУУ \"КПІ\" ({})", v)).collect(),
        }).collect(),
        source: schedule.source,
    };

    HttpResponse::Ok().json(schedule)
}

#[get("/groups")]
async fn groups() -> impl Responder {
    info!("groups list request");

    GROUPS_LIST_REQUESTS.inc();

    let client = reqwest::Client::new();

    let database = match database_connection().await {
        Ok(v) => v,
        Err(err) => {
            error!("failed to connect to database: {}", err);
            return HttpResponse::InternalServerError().body("internal_server_error");
        }
    };

    let total_groups: i64 = match total_groups_saved(&database).await {
        Ok(v) => v,
        Err(err) => {
            error!("failed to read groups count from schedule_groups: {}", err);
            return HttpResponse::InternalServerError().body("internal_server_error");
        }
    };

    let groups = if total_groups == 0 {
        let groups = rozklad_parser::get_groups(&client).await;
        info!("got {} groups from parser", groups.len());
        for group in &groups {
            if let Err(err) = add_group(&database, group).await {
                error!("failed to save entry to database: {}", err);
            }
        }
        info!("done saving groups to database");

        groups
    } else {
        let mut groups = Vec::new();

        let res = match database.query("select group_name from schedule_groups", &[]).await {
            Ok(v) => v,
            Err(err) => {
                error!("failed to read groups from schedule_groups: {}", err);
                return HttpResponse::InternalServerError().body("internal_server_error");
            }
        };

        for row in res {
            groups.push(row.get("group_name"));
        }

        groups
    };

    HttpResponse::Ok().json(groups)
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    "ok"
}

#[get("/metrics")]
async fn metrics() -> impl Responder {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    if let Err(err) = encoder.encode(&metric_families, &mut buffer) {
        error!("failed to write metrics: {}", err);
        return HttpResponse::InternalServerError().body("failed to write metrics");
    }

    let encoded = match String::from_utf8(buffer) {
        Ok(v) => v,
        Err(err) => {
            error!("failed to encode metrics: {}", err);
            return HttpResponse::InternalServerError().body("failed to encode metrics");
        }
    };

    HttpResponse::Ok().body(encoded)
}

#[get("/subjects")]
async fn subject_id_by_name(subject_name: web::Query<SubjectName>) -> impl Responder {
    let database = match database_connection().await {
        Ok(v) => v,
        Err(err) => {
            error!("failed to connect to database: {}", err);
            return HttpResponse::InternalServerError().body("internal_server_error");
        }
    };

    let res = match database.query(
        "select subject_id from subject_names where name = $1 limit 1",
        &[&subject_name.subject_name]
    ).await {
        Ok(v) => v,
        Err(err) => {
            error!("failed to execute database query: {}", err);
            return HttpResponse::InternalServerError().body("internal_server_error");
        }
    };

    HttpResponse::Ok().body(res.rows[0].get("subject_id"))
}

async fn load_group_schedule_from_database(database: &tokio_postgres::Client, group_name: &str) -> Result<Option<GroupSchedule>, PersistenceError> {
    let res = match database.query(
        "select * from schedule where group_name = $1 and updated_at > now() - interval '14 days'",
        &[&group_name]
    ).await {
        Ok(v) => v,
        Err(err) => {
            error!("failed to execute database query: {}", err);
            return Err(PersistenceError::FailedToLoad);
        }
    };

    let mut source: Option<GroupScheduleSource> = None;
    let mut entries: Vec<GroupScheduleEntry> = vec![];

    for row in res {
        entries.push(GroupScheduleEntry {
            week: ScheduleWeek::from_index(row.get::<&str, i16>("week") as u8),
            day: ScheduleDay::from_index(row.get::<&str, i16>("day") as u8),
            index: row.get::<&str, i16>("index") as u8,
            names: row.get("names"),
            lecturers: row.get("lecturers"),
            locations: row.get("locations")
        });

        if source.is_none() {
            let source_str: String = row.get("source");

            source = GroupScheduleSource::from_string(&source_str);
            if source.is_none() {
                error!("unknown from group schedule source in database: {}", &source_str);
            }
        }
    }

    Ok(source.map(|source| GroupSchedule { source: Some(source), entries }))
}
