use crate::parsers::try_parse_all_formats;
use chrono::{DateTime, Datelike, Local, TimeZone, Timelike};

/// Just a shorthand datetime fetcher, as we're going to be repeating this a lot.
fn dt(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> DateTime<Local> {
    Local.ymd(y, m, d).and_hms(h, min, s)
}

#[test]
fn test_time_formats() {
    let now = Local::now();

    assert_eq!(
        try_parse_all_formats("15:00", now).unwrap(),
        dt(now.year(), now.month(), now.day(), 15, 0, 0)
    );

    assert_eq!(
        try_parse_all_formats("15:00:34", now).unwrap(),
        dt(now.year(), now.month(), now.day(), 15, 0, 34)
    );
}

#[test]
fn test_date_formats() {
    let now = Local::now();

    assert_eq!(
        try_parse_all_formats("2018-12-12", now).unwrap(),
        dt(2018, 12, 12, now.hour(), now.minute(), now.second())
    );
}

#[test]
fn test_datetime_formats() {
    let now = Local::now();

    assert_eq!(
        try_parse_all_formats("2018-12-12 15:00:00", now).unwrap(),
        dt(2018, 12, 12, 15, 0, 0)
    );
}
