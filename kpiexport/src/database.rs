use tokio_postgres::{connect, Client, Error, NoTls};
use thiserror::Error;
use crate::config::{postgres_db, postgres_host, postgres_password, postgres_port, postgres_username};
use crate::models::schedule::SubjectId;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("postgres query failed: {0}")]
    QueryFailed(String),
}

pub async fn database_connection() -> Result<Client, Error> {
    let (client, connection) = connect(&config_str(), NoTls).await?;

    actix_rt::spawn(async move {
        if let Err(e) = connection.await {
            error!("database connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn subject_id_by_name(database: &Client, subject_name: &str) -> Result<Option<SubjectId>, DatabaseError> {
    let res = match database.query(
        "select subject_id from subject_names where name = $1 limit 1",
        &[&subject_name]
    ).await {
        Ok(v) => v,
        Err(err) => return Err(DatabaseError::QueryFailed(err.to_string())),
    };

    if res.len() == 0 {
        return Ok(None);
    }

    Ok(Some(SubjectId::new(res[0].get::<&str, i32>("subject_id"))))
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
