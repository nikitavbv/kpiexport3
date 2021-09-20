use std::io::{Result as IOResult, ErrorKind};
use std::io::Error as IOError;

use chrono::prelude::*;

use crate::database::database_connection;
use crate::rozklad_parser::get_groups;
use crate::models::groups::{delete_all_groups_transaction, add_group_transaction, total_old_groups};

pub async fn refresh_groups() -> IOResult<()> {
    let client = reqwest::Client::new();
    let mut database = match database_connection().await {
        Ok(v) => v,
        Err(err) => return IOResult::Err(IOError::new(
            ErrorKind::Other,
            format!("failed to connect to database: {}", err)
        ))
    };

    // in January, February, August and September refresh every 2-3 days, other months - every ~20 days
    let utc: DateTime<Utc> = Utc::now();
    let is_hot_month = utc.month0() <= 1 || utc.month0() == 7 || utc.month0() == 8;
    let days_diff = if is_hot_month { 2 } else { 20 };

    let old_groups = match total_old_groups(&database, days_diff).await {
        Ok(v) => v,
        Err(err) => return IOResult::Err(IOError::new(
            ErrorKind::Other,
            format!("failed to get old groups count from database: {}", err)
        ))
    };
    if old_groups == 0 {
        info!("no groups to refresh");
        return Ok(())
    }

    let groups = get_groups(&client).await;
    info!("got {} groups from parser", groups.len());
    if groups.len() > 0 {
        let transaction = database.transaction().await
            .expect("failed to start transaction");

        delete_all_groups_transaction(&transaction).await.unwrap();
        for group in &groups {
            add_group_transaction(&transaction, group).await.unwrap();
        }

        transaction.commit().await.expect("failed to commit transaction");

        info!("inserted {} groups", groups.len());
    } else {
        info!("no groups were fetched, so nothing was updated");
    }

    Ok(())
}