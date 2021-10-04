use std::io::{Result as IOResult, ErrorKind};
use std::io::Error as IOError;

use chrono::{DateTime, Utc, Datelike};
use tokio_postgres::{Client, Transaction};

use crate::database::database_connection;
use crate::models::schedule_queries::{api_groups_to_refresh, groups_with_old_schedule, remove_old_schedule_from_database_transaction, save_schedule_to_database_transaction};
use crate::rozklad::group_schedule_by_name;
use crate::rozklad_parser::Term;

pub async fn refresh_schedule() -> IOResult<()> {
    let client = reqwest::Client::new();
    let database = match database_connection().await {
        Ok(v) => v,
        Err(err) => return IOResult::Err(IOError::new(
            ErrorKind::Other,
            format!("failed to connect to database: {}", err)
        ))
    };

    // in January, February, August and September refresh every ~6 hours, other months - every ~500 hours (around once a month)
    let utc: DateTime<Utc> = Utc::now();
    let is_hot_month = utc.month0() <= 1 || utc.month0() == 7 || utc.month0() == 8;
    let hours_diff = if is_hot_month { 6 } else { 500 };

    let groups_with_api_source = match api_groups_to_refresh(&database).await {
        Ok(v) => v,
        Err(err) => return IOResult::Err(IOError::new(
            ErrorKind::Other,
            format!("failed to get groups with api source for schedule: {}", err)
        ))
    };

    if groups_with_api_source.len() > 0 {
        info!("refreshing groups with api source schedule: {}", groups_with_api_source.len());
        return refresh_schedule_for_groups(database, client, &groups_with_api_source).await;
    }

    let groups_to_refresh = match groups_with_old_schedule(&database, hours_diff).await {
        Ok(v) => v,
        Err(err) => return IOResult::Err(IOError::new(
            ErrorKind::Other,
            format!("failed to get groups with old schedule: {}", err)
        ))
    };

    if groups_to_refresh.len() > 0 {
        info!("refreshing groups with old schedule: {}", groups_to_refresh.len());
        return refresh_schedule_for_groups(database, client, &groups_to_refresh).await;
    }

    info!("looks like there is nothing to refresh");
    Ok(())
}

async fn refresh_schedule_for_groups(database: Client, client: reqwest::Client, groups_to_refresh: &Vec<String>) -> IOResult<()> {
    let mut database = database;

    for group_to_refresh in groups_to_refresh {
        info!("refreshing schedule for {}", group_to_refresh);

        let transaction = database.transaction().await
            .expect("failed to start transaction");

        refresh_schedule_for_group(&transaction, &client,group_to_refresh).await?;

        transaction.commit().await.expect("failed to commit transaction");
    }

    info!("refreshed schedule for {} groups", groups_to_refresh.len());

    Ok(())
}

async fn refresh_schedule_for_group(database: &Transaction<'_>, client: &reqwest::Client, group_name: &str) -> IOResult<()> {
    if let Err(err) = remove_old_schedule_from_database_transaction(&database, &group_name).await {
        error!("failed to remove schedule from database: {}", err);
    }

    let schedule = match group_schedule_by_name(&client, &Term::current(), &group_name).await {
        Ok(v) => v,
        Err(err) => {
            error!("failed to get group schedule: {}", err);
            return IOResult::Err(IOError::new(
                ErrorKind::Other,
                format!("failed to get group schedule: {}", err)
            ));
        }
    };

    if let Err(err) = save_schedule_to_database_transaction(&database, &group_name, &schedule).await {
        return IOResult::Err(IOError::new(
            ErrorKind::Other,
            format!("failed to save group schedule to database {}", err)
        ));
    }

    info!("refreshed schedule for {}", group_name);

    Ok(())
}