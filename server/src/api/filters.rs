use time::{format_description::FormatItem, macros::format_description, OffsetDateTime};
use tracing_unwrap::ResultExt;

pub fn format_date(date: &OffsetDateTime) -> ::askama::Result<String> {
    static DATE_FORMAT: &[FormatItem] =
        format_description!("[year].[month].[day]. [hour]:[minute]");
    let formatted = date
        .format(DATE_FORMAT)
        .expect_or_log("date formatting failed, it should not as format is compile time verified");
    Ok(formatted)
}
