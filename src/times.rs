use std::str::FromStr;

use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

pub static YMD: &str = "%Y%m%d";
pub static YMD_PATH: &str = "%Y/%m/%d";
pub static YMD_M: &str = "%Y-%m-%d";
pub static DATE_TIME_M: &str = "%Y-%m-%d  %H:%M:%S";
pub static YMDH_PATH: &str = "%Y/%m/%d/%H";
pub static HMM: &str = "%H:%M:%S";
pub static YMD_PATH_HMM: &str = "%Y/%m/%d %H:%M:%S";
pub static ZERO_HMM: &str = "00:00:00";
pub static SPACE: &str = " ";

pub fn utc(i: i32) -> FixedOffset {
    FixedOffset::east(i * 3600)
}

pub fn utc8() -> FixedOffset {
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

pub fn to_utc8<T: TimeZone>(t: DateTime<T>) -> DateTime<Local> {
    let t = t.timestamp();
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

pub fn day_path_range(start: &str, end: &str) -> Vec<String> {
    let start = &add_zero(start);
    let end = &add_zero(end);
    let t = Local.timestamp_millis(
        NaiveDateTime::parse_from_str(start, YMD_PATH_HMM)
            .unwrap()
            .timestamp_millis(),
    );
    let mut start = to_utc8(t);
    let end = to_utc8(
        Local.timestamp_millis(
            NaiveDateTime::parse_from_str(end, YMD_PATH_HMM)
                .unwrap()
                .timestamp_millis(),
        ),
    );
    let mut ret = vec![];
    while start.le(&end) {
        ret.push(start.format(YMD_PATH).to_string());
        start = add_day(&start, 1)
    }
    ret
}

pub fn add_zero(date: &str) -> String {
    date.to_string() + SPACE + ZERO_HMM
}

pub fn parse_year(arg: &str) -> NaiveDateTime {
    let d = NaiveDate::from_ymd(arg.parse().unwrap_or_default(), 1, 1);
    let t = NaiveTime::from_hms(0, 0, 0);
    NaiveDateTime::new(d, t)
}
pub fn zero_naive_time() -> NaiveTime {
    NaiveTime::from_hms(0, 0, 0)
}
pub fn zero_naive_date() -> NaiveDate {
    NaiveDate::from_yo(0, 0)
}
pub fn zero_naive_date_time() -> NaiveDateTime {
    NaiveDateTime::new(zero_naive_date(), zero_naive_time())
}
pub fn from_ymd(y: i32, m: u32, d: u32) -> NaiveDateTime {
    NaiveDateTime::new(NaiveDate::from_ymd(y, m, d), zero_naive_time())
}

pub fn parse_date(arg: &str) -> NaiveDate {
    if arg.len() < 10 {
        return zero_naive_date();
    }
    let year = arg[0..4].parse().unwrap_or_default();
    let mon = arg[5..7].parse().unwrap_or_default();
    let day = arg[8..10].parse().unwrap_or_default();
    return NaiveDate::from_ymd(year, mon, day);
}

pub fn parse_time(arg: &str) -> NaiveTime {
    if arg.len() < 8 {
        return zero_naive_time();
    }
    let hour = arg[0..2].parse().unwrap_or_default();
    let min = arg[4..5].parse().unwrap_or_default();
    let sec = arg[6..8].parse().unwrap_or_default();
    let ms = {
        if arg.len() > 9 {
            arg[9..arg.len()].parse().unwrap_or_default()
        } else {
            0
        }
    };
    return NaiveTime::from_hms_micro(hour, min, sec, ms);
}

pub fn parse_date_time(arg: &str) -> NaiveDateTime {
    let date = parse_date(&arg);
    let mut time = zero_naive_time();
    if arg.len() >= 19 {
        time = parse_time(&arg[11..arg.len()]);
    }
    return NaiveDateTime::new(date, time);
}

#[test]
fn test_time() {
    // let t = parse_date("2022/10/24");
    // let start = t.and_hms(0, 0, 0).and_local_timezone(utc8());
    // let end = t.and_hms(23, 59, 59).and_local_timezone(utc8());
    // println!("{start:?}\n{end:?}");

    let a = unix_2_date_time(1666454400);
    println!("{a:?}");
    let t = unix_2_date_time(1666540799);
    println!("{t:?}");
    
}
