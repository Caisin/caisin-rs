use std::str::FromStr;

use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};

pub static YMD: &str = "%Y%m%d";
pub static YMD_PATH: &str = "%Y/%m/%d";
pub static YMD_M: &str = "%Y-%m-%d";
pub static DATE_TIME_M: &str = "%Y-%m-%d  %H:%M:%S";
pub static YMDH_PATH: &str = "%Y/%m/%d/%H";

fn utc(i: i32) -> FixedOffset {
    FixedOffset::east(i * 3600)
}

fn utc8() -> FixedOffset {
    utc(8)
}

/// unix时间转datetime
pub fn unix_str_2_date_time(s: &str) -> DateTime<Local> {
    let secs = i64::from_str(s).unwrap();
    unix_2_date_time(secs)
}

/// unix时间转datetime
pub fn unix_2_date_time(s: i64) -> DateTime<Local> {
    let naive_datetime = NaiveDateTime::from_timestamp(s, 0);
    let datetime: DateTime<Local> = DateTime::from_utc(naive_datetime, utc8());
    datetime
}
/// 新增日期
pub fn add_day(t: &DateTime<Local>, day: i64) -> DateTime<Local> {
    let t = t.timestamp() + day * 24 * 60 * 60;
    let datetime = NaiveDateTime::from_timestamp(t, 0);
    DateTime::from_utc(datetime, utc8())
}

pub fn get_yyyymmdd(t: &DateTime<Local>) -> String {
    t.format(YMD).to_string()
}

pub fn get_yyyy_mm_dd(t: &DateTime<Local>) -> String {
    t.format(YMD_M).to_string()
}

pub fn get_date_time(t: &DateTime<Local>) -> String {
    t.format(DATE_TIME_M).to_string()
}

pub fn get_hour_path(t: &DateTime<Local>) -> String {
    t.format(YMDH_PATH).to_string()
}

pub fn get_date_path(t: &DateTime<Local>) -> String {
    t.format(YMD_PATH).to_string()
}

pub fn get_sys_date_path() -> String {
    Local::now().format(YMD_PATH).to_string()
}
pub fn get_yesterday_date_path() -> String {
    get_yesterday().format(YMD_PATH).to_string()
}

pub fn get_sys_date_time() -> String {
    Local::now().format(DATE_TIME_M).to_string()
}

pub fn get_yesterday() -> DateTime<Local> {
    add_day(&sys_date_time(), -1)
}

pub fn sys_date_time() -> DateTime<Local> {
    let naive_utc = Local::now().naive_utc();
    DateTime::from_utc(naive_utc, utc8())
}

#[test]
fn test_time() {
    // let now = Local::now();
    // let timestamp = now.timestamp();
    let timestamp = 1666404492;
    let u = unix_2_date_time(timestamp);
    let l = unix_2_date_time(timestamp);
    println!("{:?}\n{:?}", u, l);
}
