use time::{PrimitiveDateTime as DateTime, UtcDateTime};

// Returns a DateTime one billion seconds after start.
fn _after(start: DateTime) -> Result<DateTime, time::error::ComponentRange> {
    let seconds = start.as_utc().unix_timestamp();
    let unix_new_date = seconds + 1_000_000_000;
    let new_date = UtcDateTime::from_unix_timestamp(unix_new_date)?;
    Ok(DateTime::new(new_date.date(), new_date.time()))
}

pub fn after(start: DateTime) -> DateTime {
    _after(start).unwrap()
}
