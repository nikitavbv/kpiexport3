use chrono::{DateTime, Utc};
use crate::errors::RozkladParseError;

const VIEW_SCHEDULE_PREFIX: &'static str = "ViewSchedule.aspx?g=";

pub fn group_id_from_url(url: &str) -> Result<String, RozkladParseError> {
    match url.find(VIEW_SCHEDULE_PREFIX) {
        None => Err(RozkladParseError::FailedToParseGroupId {}),
        Some(index) => Ok(url[index + VIEW_SCHEDULE_PREFIX.len()..].to_string())
    }
}

pub fn is_hot_month() -> bool {
    // in January, February, August and September refresh every 2-3 days, other months - every ~20 days
    let utc: DateTime<Utc> = Utc::now();
    utc.month0() <= 1 || utc.month0() == 7 || utc.month0() == 8
}