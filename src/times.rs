use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub static YMD: &str = "%Y%m%d";
pub static YMD_PATH: &str = "%Y/%m/%d";
pub static YMD_M: &str = "%Y-%m-%d";
pub static DATE_TIME_M: &str = "%Y-%m-%d  %H:%M:%S";
pub static YMDH_PATH: &str = "%Y/%m/%d/%H";

/// unix时间转datetime
pub fn unix_str_2_date_time(s: &str) -> DateTime<Utc> {
    let secs = i64::from_str(s).unwrap();
    let naive_datetime = NaiveDateTime::from_timestamp(secs, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    datetime
}
pub fn get_yyyymmdd(t: &DateTime<Utc>) -> String {
    t.format(YMD).to_string()
}

pub fn get_yyyy_mm_dd(t: &DateTime<Utc>) -> String {
    t.format(YMD_M).to_string()
}

pub fn get_date_time(t: &DateTime<Utc>) -> String {
    t.format(DATE_TIME_M).to_string()
}

pub fn get_hour_path(t: &DateTime<Utc>) -> String {
    t.format(YMDH_PATH).to_string()
}

pub fn get_date_path(t: &DateTime<Utc>) -> String {
    t.format(YMD_PATH).to_string()
}

pub fn get_sys_date_path() -> String {
    Local::now().format(YMD_PATH).to_string()
}

pub fn get_sys_date_time() -> String {
    Local::now().format(DATE_TIME_M).to_string()
}
