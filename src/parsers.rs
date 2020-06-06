use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, ParseError, TimeZone, Timelike};
use std::process;

/// Eager datetime parsing for given arguments, testing multiple date and time formats and only
/// quitting if absolutely nothing matches.
pub fn parse_arg_or_exit(arg: &str, now: &DateTime<Local>) -> DateTime<Local> {
    match try_parse_times(arg, &now).or_else(|_err| try_parse_dates(arg, &now)) {
        Ok(val) => val,
        Err(err) => {
            println!("Unable to parse `{}` into datetime: {}.", arg, err);
            process::exit(1);
        }
    }
}

/// Tries to parse given argument through multiple different time formats and format a locale-aware
/// current datetime using given `now`.
fn try_parse_times(arg: &str, now: &DateTime<Local>) -> Result<DateTime<Local>, ParseError> {
    NaiveTime::parse_from_str(&arg, "%T")
        .or_else(|_err| NaiveTime::parse_from_str(&arg, "%R"))
        .and_then(|val| {
            Ok(Local.ymd(now.year(), now.month(), now.day()).and_hms(
                val.hour(),
                val.minute(),
                val.second(),
            ))
        })
}

/// Tries to parse given argument through multiple different date formats and format a locale-aware
/// current datetime using given `now`.
fn try_parse_dates(arg: &str, now: &DateTime<Local>) -> Result<DateTime<Local>, ParseError> {
    NaiveDate::parse_from_str(&arg, "%F").and_then(|val| {
        Ok(Local.ymd(val.year(), val.month(), val.day()).and_hms(
            now.hour(),
            now.minute(),
            now.second(),
        ))
    })
}
