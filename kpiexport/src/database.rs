use crate::config::{postgres_db, postgres_host, postgres_password, postgres_port, postgres_username};
use tokio_postgres::{connect, Client, Error, NoTls};
use crate::models::schedule::SubjectId;

pub async fn database_connection() -> Result<Client, Error> {
    let (client, connection) = connect(&config_str(), NoTls).await?;

    actix_rt::spawn(async move {
        if let Err(e) = connection.await {
            error!("database connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn subject_id_by_name(client: &Client, subject_name: &str) -> Option<SubjectId> {
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
}

pub fn config_str() -> String {
    // see https://docs.rs/postgres/0.16.0-rc.2/postgres/config/struct.Config.html
    format!(
        "host={} port={} user={} password={} dbname={}",
        postgres_host(),
        postgres_port(),
        postgres_username(),
        postgres_password(),
        postgres_db()
    )
}
