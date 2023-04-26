use crate::error::Result;

/// Get an ISO8601 timestamp
pub fn get_timestamp() -> Result<String> {
    use time::{macros::format_description, OffsetDateTime};

    OffsetDateTime::now_local()?
        .format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .map_err(Into::into)
}
