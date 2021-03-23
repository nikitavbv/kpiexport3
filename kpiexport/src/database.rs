use crate::config::{postgres_db, postgres_host, postgres_password, postgres_port, postgres_username};
use tokio_postgres::{connect, Client, Error, NoTls};

pub async fn database_connection() -> Result<Client, Error> {
    let (client, connection) = connect(&config_str(), NoTls).await?;

    actix_rt::spawn(async move {
        if let Err(e) = connection.await {
            error!("database connection error: {}", e);
        }
    });

    Ok(client)
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
