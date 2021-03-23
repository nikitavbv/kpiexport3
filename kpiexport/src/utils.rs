use crate::errors::RozkladParseError;

const VIEW_SCHEDULE_PREFIX: &'static str = "ViewSchedule.aspx?g=";

pub fn group_id_from_url(url: &str) -> Result<String, RozkladParseError> {
    match url.find(VIEW_SCHEDULE_PREFIX) {
        None => Err(RozkladParseError::FailedToParseGroupId {}),
        Some(index) => Ok(url[index + VIEW_SCHEDULE_PREFIX.len()..].to_string())
    }
}