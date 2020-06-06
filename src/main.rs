extern crate chrono;
extern crate clap;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, ParseError, TimeZone, Timelike};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::process;

// Available subcommand branches
const DAYS: &str = "days";
const HOURS: &str = "hours";
const MINUTES: &str = "minutes";
const SECONDS: &str = "seconds";
const BASE: &str = "NOT_SUBCMD";

fn print_time_difference(from: DateTime<Local>, to: DateTime<Local>, name: &str) {
    let mut difference = to.signed_duration_since(from);

    if difference.num_seconds() < 0 {
        // Swap times if value is in future. Never show negative numbers.
        //
        // NOTE: While this is breaking the semantics of `since` a bit, we'll allow it
        // for better usability. You could basically just symlink `since` -> `until`.
        difference = from.signed_duration_since(to);
    }

    match name {
        DAYS => {
            println!("{}", difference.num_days());
        }
        HOURS => {
            println!("{}", difference.num_hours());
        }
        MINUTES => {
            println!("{}", difference.num_minutes());
        }
        SECONDS => {
            println!("{}", difference.num_seconds());
        }
        _ => {
            // GUESS HERE
            let hours = difference.num_hours();
            let mins = difference.num_minutes() % 60;

            println!("{:02}:{:02}", hours, mins);
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
        .help("How much time has passed since date or time")
        .required(false)
        .index(1);

    let to: Arg = Arg::with_name("to")
        .help("Use to set custom time range. Default is current datetime.")
        .required(false)
        .index(2);

    let about = "
Fetch time difference between <from> and <to>.

If no parameters are given, will return time since UNIX epoch.
Missing <to> argument will always default to current date/time.
    ";

    let matches = App::new("since")
        .about(about)
        .version("v0.8")
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .arg(&from)
        .arg(&to)
        .subcommand(
            SubCommand::with_name(DAYS)
                .about("Return output in days")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(HOURS)
                .about("Return output in hours")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(MINUTES)
                .about("Return output in minutes")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(SECONDS)
                .about("Return output in seconds")
                .arg(&from)
                .arg(&to),
        )
        .get_matches();

    match matches.subcommand() {
        (subcmd, Some(sub_matches)) => handle_args(subcmd, sub_matches),
        _ => handle_args(BASE, &matches),
    }
}
