use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};

/// unix时间转datetime
pub fn unix_str_2_date_time(s: &str) -> DateTime<Utc> {
    let secs = i64::from_str(s).unwrap();
    let naive_datetime = NaiveDateTime::from_timestamp(secs, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    datetime
}
pub fn get_yyyymmdd(t: &DateTime<Utc>) -> String {
    t.format("%Y%m%d").to_string()
}

pub fn get_yyyy_mm_dd(t: &DateTime<Utc>) -> String {
    t.format("%Y-%m-%d").to_string()
}

pub fn get_date_time(t: &DateTime<Utc>) -> String {
    t.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_hour_path(t: &DateTime<Utc>) -> String {
    t.format("%Y/%m/%d/%H").to_string()
}

pub fn get_date_path(t: &DateTime<Utc>) -> String {
    t.format("%Y/%m/%d").to_string()
}
