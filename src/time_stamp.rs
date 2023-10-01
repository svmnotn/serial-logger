/// Get an ISO8601 timestamp
pub fn get_timestamp() -> String {
    use chrono::Local;
    format!("{}", Local::now())
}
