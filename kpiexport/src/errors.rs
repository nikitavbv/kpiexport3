use custom_error::custom_error;

custom_error!{pub RozkladParseError
    RequestFailed { source: reqwest::Error } = "request to rozklad failed: {}",
    HtmlParseFailed { description: String } = "html parse failed: {}",
    RozkladErrored = "rozklad errored",
    RozkladApiErrored = "rozklad api errored",
    FailedToParseGroupId = "failed to parse group id",
    RozkladParseError { source: std::num::ParseIntError } = "failed to parse number",
}

custom_error! {pub PersistenceError
    FailedToSave = "failed to save schedule to database",
    FailedToLoad = "failed to load schedule from database"
}