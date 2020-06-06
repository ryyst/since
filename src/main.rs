extern crate chrono;
extern crate clap;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, ParseError, TimeZone, Timelike};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::process;

// Available subcommand branches
const YEARS: &str = "years";
const MONTHS: &str = "months";
const WEEKS: &str = "weeks";
const DAYS: &str = "days";
const HOURS: &str = "hours";
const MINUTES: &str = "minutes";
const SECONDS: &str = "seconds";
const BASE: &str = "NOT_SUBCMD";

fn print_time_difference(from: DateTime<Local>, to: DateTime<Local>, name: &str) {
    let difference = to.signed_duration_since(from);
    // NOTE:
    // All values are printed in absolutes, as to not show negative number for values in
    // future. While this is breaking the semantics of `since` a bit, we'll allow it for
    // better usability. You could basically just symlink `since` -> `until`.

    match name {
        YEARS => {
            let year_diff = from.year() - to.year();
            println!("{}", year_diff.abs());
        }
        MONTHS => {
            // Individual typecasting is necessary to
            // a) compile at all
            // b) not panic from subtraction overflow
            let from_month = from.month() as i32;
            let to_month = to.month() as i32;

            // TODO: Figure out if we want a more precise formula here.
            let months: i32 = (from.year() - to.year()) * 12 + from_month - to_month;
            println!("{}", months.abs());
        }
        WEEKS => {
            println!("{}", difference.num_weeks().abs());
        }
        DAYS => {
            println!("{}", difference.num_days().abs());
        }
        HOURS => {
            println!("{}", difference.num_hours().abs());
        }
        MINUTES => {
            println!("{}", difference.num_minutes().abs());
        }
        SECONDS => {
            println!("{}", difference.num_seconds().abs());
        }
        _ => {
            // GUESS HERE
            let hours = difference.num_hours();
            let mins = difference.num_minutes() % 60;

            println!("{:02}:{:02}", hours.abs(), mins.abs());
        }
    }
}

fn print_formatted_epoch(subcmd: &str) {
    let epoch = Local::now().timestamp();
    match subcmd {
        DAYS => println!("{}", epoch / 60 / 60 / 24),
        HOURS => println!("{}", epoch / 60 / 60),
        MINUTES => println!("{}", epoch / 60),
        _ => println!("{}", epoch),
    }
}

/// Eager datetime parsing for given arguments, testing multiple date and time formats and only
/// quitting if absolutely nothing matches.
fn try_parse_arg(arg: &str, now: &DateTime<Local>) -> DateTime<Local> {
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

fn handle_args(subcmd: &str, matches: &ArgMatches) {
    let now = Local::now();

    let from: DateTime<Local> = match matches.value_of("from") {
        Some(val) => try_parse_arg(val, &now),
        None => {
            print_formatted_epoch(subcmd);
            process::exit(0);
        }
    };

    let to: DateTime<Local> = match matches.value_of("to") {
        Some(val) => try_parse_arg(val, &now),
        None => now,
    };

    print_time_difference(from, to, subcmd);
}

fn main() {
    let from: Arg = Arg::with_name("from")
        .help("Start time or date.")
        .required(false)
        .index(1);

    let to: Arg = Arg::with_name("to")
        .help("End time or date, for custom range. Default is current datetime.")
        .required(false)
        .index(2);

    let about = "
Fetch time difference between <from> and <to>.

If no parameters are given, will return time since UNIX epoch.
Missing <to> argument will always default to current date/time.

All values are generally rounded down.
    ";

    let matches = App::new("since")
        .about(about)
        .version("v0.8")
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(&from)
        .arg(&to)
        .subcommand(
            SubCommand::with_name(YEARS)
                .about("Print the output in years")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(MONTHS)
                .about("Print the output in months (approx)")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(WEEKS)
                .about("Print the output in weeks (approx)")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(DAYS)
                .about("Print the output in days")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(HOURS)
                .about("Print the output in hours")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(MINUTES)
                .about("Print the output in minutes")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(SECONDS)
                .about("Print the output in seconds")
                .arg(&from)
                .arg(&to),
        )
        .get_matches();

    match matches.subcommand() {
        (subcmd, Some(sub_matches)) => handle_args(subcmd, sub_matches),
        _ => handle_args(BASE, &matches),
    }
}
