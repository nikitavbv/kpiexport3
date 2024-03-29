use crate::rozklad_parser;
use crate::rozklad_api;
use crate::models::schedule::*;
use crate::errors::RozkladParseError;
use crate::rozklad_parser::Term;

// get schedule by group name
pub async fn group_schedule_by_name(client: &reqwest::Client, term: &Term, name: &str) -> Result<GroupSchedule, RozkladParseError> {
    match rozklad_parser::group_id_by_name(&client, &name).await {
        Ok(id) => {
            info!("group id is: {}", id);

            match rozklad_parser::group_schedule(&client, term, &id).await.map(|v| v.0) { 
                Ok(schedule) => Ok(schedule),
                Err(err) => {
                    error!("failed to get rozklad using parser: {}", err);
                    rozklad_api::group_schedule(&client, &name).await
                }
            }
        }
        Err(err) => {
            error!("failed to get rozklad using parser: {}", err);
            rozklad_api::group_schedule(&client, &name).await
        }
    }
}
