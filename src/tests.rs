// Import with a shorthand for readability.
use crate::parsers::try_parse_all_formats as parse;
use chrono::{DateTime, Datelike, Local, TimeZone, Timelike};

//
// A couple of shorthands, implemented with the same basic logic as the parsers:
// fill out any missing date/time data with the value of `now`.
//
fn dt(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> DateTime<Local> {
    Local.ymd(y, m, d).and_hms(h, min, s)
}

fn local_time(h: u32, m: u32, s: u32, now: DateTime<Local>) -> DateTime<Local> {
    Local
        .ymd(now.year(), now.month(), now.day())
        .and_hms(h, m, s)
}

fn local_date(y: i32, m: u32, d: u32, now: DateTime<Local>) -> DateTime<Local> {
    Local
        .ymd(y, m, d)
        .and_hms(now.hour(), now.minute(), now.second())
}

//
// NOTE:
// We'll be testing the public parse-everything function instead of all the individual functions
// separately, because that's the only one we honestly care about.
//

//
// TIMES
//
#[test]
fn test_valid_time_formats_return_correct_datetimes() {
    let now = Local::now();

    assert_eq!(parse("15:00", now).unwrap(), local_time(15, 0, 0, now));
    assert_eq!(parse("15:00:34", now).unwrap(), local_time(15, 0, 34, now));
}

#[test]
fn test_invalid_time_formats_return_errors() {
    let now = Local::now();

    assert!(parse("15.00", now).is_err());
    assert!(parse("15:00.0", now).is_err());
    assert!(parse("T15:00", now).is_err());
    assert!(parse("15:00T", now).is_err());
}

#[test]
fn test_invalid_time_values_return_errors() {
    let now = Local::now();

    assert!(parse("f0:f0", now).is_err());
    assert!(parse("24:00", now).is_err());
    assert!(parse("24:01", now).is_err());
    assert!(parse("15:60", now).is_err());
    assert!(parse("-15:30", now).is_err());
    assert!(parse("15:-30", now).is_err());
}

//
// DATES
//
#[test]
fn test_valid_date_formats_return_correct_datetimes() {
    let now = Local::now();
    let christmas = local_date(2018, 12, 24, now);

    assert_eq!(parse("2018-12-24", now).unwrap(), christmas);
    assert_eq!(parse("2018/12/24", now).unwrap(), christmas);
    assert_eq!(parse("2018.12.24", now).unwrap(), christmas);
    assert_eq!(parse("24-12-2018", now).unwrap(), christmas);
    assert_eq!(parse("24/12/2018", now).unwrap(), christmas);
    assert_eq!(parse("24.12.2018", now).unwrap(), christmas);
    assert_eq!(parse("2018 Dec 24", now).unwrap(), christmas);
    assert_eq!(parse("2018 December 24", now).unwrap(), christmas);
    assert_eq!(parse("24 Dec 2018", now).unwrap(), christmas);

    assert_eq!(
        parse("24.12.9999", now).unwrap(),
        local_date(9999, 12, 24, now)
    );
}

#[test]
fn test_invalid_date_formats_return_errors() {
    let now = Local::now();

    // Different separators
    assert!(parse("2018|12|24", now).is_err());
    assert!(parse("2018:12:24", now).is_err());

    // No spaces supported without %B
    assert!(parse("2018 12 24", now).is_err());
    assert!(parse("24 12 2018", now).is_err());

    // Invalid shorthands
    assert!(parse("24 Decem 24", now).is_err());
    assert!(parse("24 De 24", now).is_err());
}

#[test]
fn test_invalid_date_values_return_errors() {
    let now = Local::now();

    assert!(parse("f0.12.2018", now).is_err());
    assert!(parse("-24.12.2018", now).is_err());

    assert!(parse("24.24.2018", now).is_err());
    assert!(parse("44.12.2018", now).is_err());

    // TODO: 10000 seems to be our limit, check out what this depends upon?
    assert!(parse("24.12.10000", now).is_err());
}

//
// DATETIMES
//
#[test]
fn test_valid_datetime_formats_return_correct_datetimes() {
    let now = Local::now();
    let christmas = dt(2018, 12, 24, 15, 30, 45);
    let secondless = dt(2018, 12, 24, 15, 30, 0);

    // List copied straight from parsers.rs
    assert_eq!(parse("24 December 2018 15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("2018 Dec 24 15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("24 Dec 2018 15:30", now).unwrap(), secondless);
    assert_eq!(parse("2018 December 24 15:30", now).unwrap(), secondless);
    // Dashes
    assert_eq!(parse("2018-12-24 15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("2018-12-24 15:30", now).unwrap(), secondless);
    assert_eq!(parse("24-12-2018 15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("24-12-2018 15:30", now).unwrap(), secondless);
    // Dots
    assert_eq!(parse("2018.12.24 15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("2018.12.24 15:30", now).unwrap(), secondless);
    assert_eq!(parse("24.12.2018 15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("24.12.2018 15:30", now).unwrap(), secondless);
    // Slashes
    assert_eq!(parse("2018/12/24 15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("2018/12/24 15:30", now).unwrap(), secondless);
    assert_eq!(parse("24/12/2018 15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("24/12/2018 15:30", now).unwrap(), secondless);
    // Dashes, dots & slashes, but with a T
    assert_eq!(parse("2018-12-24T15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("2018-12-24T15:30", now).unwrap(), secondless);
    assert_eq!(parse("24-12-2018T15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("24-12-2018T15:30", now).unwrap(), secondless);
    assert_eq!(parse("2018.12.24T15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("2018.12.24T15:30", now).unwrap(), secondless);
    assert_eq!(parse("24.12.2018T15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("24.12.2018T15:30", now).unwrap(), secondless);
    assert_eq!(parse("2018/12/24T15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("2018/12/24T15:30", now).unwrap(), secondless);
    assert_eq!(parse("24/12/2018T15:30:45", now).unwrap(), christmas);
    assert_eq!(parse("24/12/2018T15:30", now).unwrap(), secondless);

    // Limits
    assert_eq!(
        parse("0-12-24T15:30:45", now).unwrap(),
        dt(0, 12, 24, 15, 30, 45)
    );
    assert_eq!(
        parse("9999-12-24T15:30:45", now).unwrap(),
        dt(9999, 12, 24, 15, 30, 45)
    );
}

#[test]
fn test_invalid_datetime_formats_return_errors() {
    let now = Local::now();

    // Only specific separators allowed
    assert!(parse("2018-12-24-15:00:00", now).is_err());
    assert!(parse("2018-12-24|15:00:00", now).is_err());

    // Reverse order is not supported *yet*
    assert!(parse("15:00:00 2018-12-24", now).is_err());
}

#[test]
fn test_invalid_datetime_values_return_errors() {
    let now = Local::now();

    // Invalid time values
    assert!(parse("2018-12-24 f0:f0", now).is_err());
    assert!(parse("2018-12-24 24:00", now).is_err());
    assert!(parse("2018-12-24 24:01", now).is_err());
    assert!(parse("2018-12-24 15:60", now).is_err());
    assert!(parse("2018-12-24 -15:30", now).is_err());
    assert!(parse("2018-12-24 15:-30", now).is_err());

    // Invalid date values
    assert!(parse("f0.12.2018 15:00", now).is_err());
    assert!(parse("-24.12.2018 15:00", now).is_err());
    assert!(parse("24.24.2018 15:00", now).is_err());
    assert!(parse("44.12.2018 15:00", now).is_err());

    // Limits
    assert!(parse("-1-24-24 15:00:00", now).is_err());
    assert!(parse("10000-24-24 15:00:00", now).is_err());
}
