/// Format an RFC3339 timestamp like "2025-11-24T12:58:04.949266211Z" into a friendly string.
/// If the timestamp is in the current year, the year is omitted (e.g., "Nov 24");
/// otherwise it includes the year (e.g., "Nov 24, 2025").
#[allow(dead_code)]
pub fn format_timestamp_short(ts: &str) -> Option<String> {
    use time::format_description::well_known::Rfc3339;
    use time::format_description::FormatItem;
    use time::{format_description, OffsetDateTime};

    let parsed = OffsetDateTime::parse(ts, &Rfc3339).ok()?;
    let now_year = OffsetDateTime::now_utc().year();

    let fmt: Vec<FormatItem> = if parsed.year() == now_year {
        format_description::parse("[month repr:short] [day padding:none]").ok()?
    } else {
        format_description::parse("[month repr:short] [day padding:none], [year]").ok()?
    };

    parsed.format(&fmt).ok()
}
