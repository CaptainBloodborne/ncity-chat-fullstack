use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::application::{AppError, AppResult};

pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

pub fn format_time<T: TimeZone>(time: DateTime<T>) -> String {
    time.to_rfc3339()
}

pub fn now_utc_plus_sec_str(sec: i64) -> String {
    let t = utc_now() + Duration::seconds(sec);

    format_time(t)
}

pub fn parse_utc(time: &str) -> AppResult<DateTime<Utc>> {
    let t = DateTime::parse_from_rfc3339(time)
        .map_err(|_|AppError::Internal)?;

    Ok(t.to_utc())
}

pub enum TimeError {
    ParsingError
}