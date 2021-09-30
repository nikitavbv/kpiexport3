use tokio_postgres::{Client, Transaction};

pub async fn total_groups_saved(database: &Client) -> Result<i64, tokio_postgres::Error> {
    database.query_one("select count(*) from schedule_groups", &[]).await
        .map(|v| v.get("count"))
}

pub async fn total_old_groups(database: &Client, days_diff: i64) -> Result<i64, tokio_postgres::Error> {
    database.query_one(
        "select count(*) from schedule_groups where inserted_at <= date_trunc('day', NOW() - cast($1::text as interval))",
        &[&format!("{} days", days_diff)]
    ).await.map(|v| v.get("count"))
}

pub async fn delete_all_groups(database: &Client) -> Result<(), tokio_postgres::Error> {
    database.execute("delete from schedule_groups where 1 = 1", &[]).await
        .map(|v| ())
}

pub async fn delete_all_groups_transaction(database: &Transaction<'_>) -> Result<(), tokio_postgres::Error> {
    database.execute("delete from schedule_groups where 1 = 1", &[]).await
        .map(|v| ())
}

pub async fn add_group(database: &Client, group_name: &str) -> Result<(), tokio_postgres::Error> {
    database.execute("insert into schedule_groups (group_name) values ($1)", &[&group_name]).await
        .map(|v| ())
}

pub async fn add_group_transaction<'a>(database: &Transaction<'_>, group_name: &str) -> Result<(), tokio_postgres::Error> {
    database.execute("insert into schedule_groups (group_name) values ($1)", &[&group_name]).await
        .map(|v| ())
}