use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, ParseError, TimeZone, Timelike};
use std::process;

/// Eager datetime parsing for given arguments, testing multiple date and time formats and only
/// quitting if absolutely nothing matches.
pub fn parse_arg_or_exit(arg: &str, now: &DateTime<Local>) -> DateTime<Local> {
    match try_parse_times(arg, &now)
        .or_else(|_err| try_parse_dates(arg, &now))
        .or_else(|_err| try_parse_datetimes(arg))
    {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Unable to parse `{}` into datetime: {}.", arg, err);
            process::exit(1);
        }
    }
}

/// Tries to parse given argument through basic timestamp formats and create a locale-aware current
/// datetime using the provided `now`.
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

/// Tries to parse given argument through multiple different date formats and create a locale-aware
/// current datetime using the provided `now`.
fn try_parse_dates(arg: &str, now: &DateTime<Local>) -> Result<DateTime<Local>, ParseError> {
    // Try to go through the formats in the order of (entirely subjective) "commonness"
    NaiveDate::parse_from_str(&arg, "%Y-%m-%d")
        .or_else(|_err| NaiveDate::parse_from_str(&arg, "%Y/%m/%d"))
        .or_else(|_err| NaiveDate::parse_from_str(&arg, "%Y.%m.%d"))
        .or_else(|_err| NaiveDate::parse_from_str(&arg, "%d-%m-%Y"))
        .or_else(|_err| NaiveDate::parse_from_str(&arg, "%d/%m/%Y"))
        .or_else(|_err| NaiveDate::parse_from_str(&arg, "%d.%m.%Y"))
        .or_else(|_err| NaiveDate::parse_from_str(&arg, "%Y %B %d")) // %B == July || Jul
        .or_else(|_err| NaiveDate::parse_from_str(&arg, "%d %B %Y"))
        .and_then(|val| {
            Ok(Local.ymd(val.year(), val.month(), val.day()).and_hms(
                now.hour(),
                now.minute(),
                now.second(),
            ))
        })
}

/// Tries to parse given argument through multiple different datetime formats and create a
/// locale-aware current datetime in the current system timezone.
fn try_parse_datetimes(arg: &str) -> Result<DateTime<Local>, ParseError> {
    // For reference, full RFC 2822:  1 Jul 2003 10:52:37 +0200
    // and full RFC 3339 / ISO 8601:  1996-12-19T16:39:57-08:00

    // TODO: Figure out if we could somehow "build" all the allowed formats a bit more nicely.
    Local
        // Month name
        .datetime_from_str(&arg, "%d %B %Y %H:%M:%S")
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y %B %d %H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d %B %Y %H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y %B %d %H:%M"))
        // Dashes
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y-%m-%d %H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d-%m-%Y %H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d-%m-%Y %H:%M"))
        // Dots
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y.%m.%d %H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y.%m.%d %H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d.%m.%Y %H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d.%m.%Y %H:%M"))
        // Slashes
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y/%m/%d %H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y/%m/%d %H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d/%m/%Y %H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d/%m/%Y %H:%M"))
        // Dashes, dots & slashes, but with a T
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y-%m-%dT%H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y-%m-%dT%H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d-%m-%YT%H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d-%m-%YT%H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y.%m.%dT%H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y.%m.%dT%H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d.%m.%YT%H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d.%m.%YT%H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y/%m/%dT%H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%Y/%m/%dT%H:%M"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d/%m/%YT%H:%M:%S"))
        .or_else(|_err| Local.datetime_from_str(&arg, "%d/%m/%YT%H:%M"))

    // TODO: All of the above, but with reverse date & time..?
}
