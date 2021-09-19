use tokio_postgres::{Client, Transaction};
use crate::models::schedule::GroupSchedule;
use crate::errors::PersistenceError;

pub async fn api_groups_to_refresh(database: &Client) -> Result<Vec<String>, tokio_postgres::Error> {
    database.query("select distinct group_name from schedule where source = 'api' limit 10", &[]).await
        .map(|v| v.iter().map(|r| r.get("group_name")).collect())
}

pub async fn groups_with_old_schedule(database: &Client, hours_diff: i64) -> Result<Vec<String>, tokio_postgres::Error> {
    database.query("select distinct group_name from schedule where updated_at <= date_trunc('day', NOW() - cast($1 as interval)) limit 10", &[&format!("{} hours", hours_diff)]).await
        .map(|v| v.iter().map(|r| r.get("group_name")).collect())
}

pub async fn remove_old_schedule_from_database(database: &tokio_postgres::Client, group_name: &str) -> Result<(), tokio_postgres::Error> {
    database.execute("delete from schedule where group_name = $1", &[&group_name]).await.map(|_v| ())
}

pub async fn remove_old_schedule_from_database_transaction(database: &Transaction<'_>, group_name: &str) -> Result<(), tokio_postgres::Error> {
    database.execute("delete from schedule where group_name = $1", &[&group_name]).await.map(|_v| ())
}

//noinspection DuplicatedCode
pub async fn save_schedule_to_database(database: &tokio_postgres::Client, group_name: &str, schedule: &GroupSchedule) -> Result<(), PersistenceError> {
    for entry in &schedule.entries {
        let week_index: i16 = entry.week.to_index() as i16;
        let day_index: i16 = entry.day.to_index() as i16;
        let index: i16 = entry.index as i16;

        if let Err(err) = database.execute(
            "insert into schedule (group_name, source, week, day, index, names, lecturers, locations) values ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[&group_name, &schedule.source.as_ref().unwrap().to_string(), &week_index, &day_index, &index, &entry.names, &entry.lecturers, &entry.locations]
        ).await {
            error!("failed to save entry to database: {}", err);
        }
    }

    Ok(())
}

//noinspection DuplicatedCode
pub async fn save_schedule_to_database_transaction(database: &Transaction<'_>, group_name: &str, schedule: &GroupSchedule) -> Result<(), PersistenceError> {
    for entry in &schedule.entries {
        let week_index: i16 = entry.week.to_index() as i16;
        let day_index: i16 = entry.day.to_index() as i16;
        let index: i16 = entry.index as i16;

        if let Err(err) = database.execute(
            "insert into schedule (group_name, source, week, day, index, names, lecturers, locations) values ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[&group_name, &schedule.source.as_ref().unwrap().to_string(), &week_index, &day_index, &index, &entry.names, &entry.lecturers, &entry.locations]
        ).await {
            error!("failed to save entry to database: {}", err);
        }
    }

    Ok(())
}