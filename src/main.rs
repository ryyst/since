mod parsers;

#[cfg(test)]
mod tests;

use crate::parsers::try_parse_all_formats;
use chrono::{DateTime, Datelike, Duration, Local, TimeZone};
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

fn calculate_month_diff(from: DateTime<Local>, to: DateTime<Local>) -> i32 {
    // Individual typecasting is necessary to
    // a) compile at all
    // b) not panic from subtraction overflow
    let from_month = from.month() as i32;
    let to_month = to.month() as i32;

    // TODO: Figure out if we want a more precise formula here.
    ((from.year() - to.year()) * 12 + from_month - to_month).abs()
}

fn calculate_year_diff(from: DateTime<Local>, to: DateTime<Local>) -> i32 {
    // TODO: Figure out if we want a more precise formula here.
    (from.year() - to.year()).abs()
}

/// Print the time difference for shorthand use.
///
/// Does some basic guessing on which format is the nicest for user to read.
fn print_shorthand_format(from: DateTime<Local>, to: DateTime<Local>, difference: Duration) {
    // TODO: Clean this up and add "and"-clauses to every option, like in hours/minutes now.
    let days = difference.num_days().abs();
    match days {
        // About two months to about two years
        63..=730 => {
            println!("{} months", calculate_month_diff(from, to));
        }
        // Two days to about two months
        2..=62 => {
            println!("{} days", difference.num_days().abs());
        }
        0..=1 => {
            let hours = difference.num_hours();
            let mins = difference.num_minutes() % 60;

            // TODO: Seconds
            if hours == 0 {
                println!("{} minutes", mins.abs());
            } else {
                println!("{} hours and {} minutes", hours.abs(), mins.abs());
            }
        }
        _ => {
            println!("{} years", calculate_year_diff(from, to));
        }
    }
}

/// Print the requested `from` and `to` arguments according to the chosen subcommand.
///
/// If no subcommand is chosen, guess which is the best format for humans to read
/// for the given time range.
fn print_time_difference(from: DateTime<Local>, to: DateTime<Local>, subcmd: &str) {
    let difference = to.signed_duration_since(from);
    // NOTE:
    // All values are printed in absolutes, as to not show negative number for values in
    // future. While this is breaking the semantics of `since` a bit, we'll allow it for
    // better usability. You could basically just symlink `since` -> `until`.

    match subcmd {
        YEARS => {
            println!("{}", calculate_year_diff(from, to));
        }
        MONTHS => {
            println!("{}", calculate_month_diff(from, to));
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
            print_shorthand_format(from, to, difference);
        }
    }
}

/// Print the UNIX timestamp according to the chosen subcommand.
fn print_formatted_epoch(subcmd: &str, now: DateTime<Local>) {
    let epoch = Local::now().timestamp();
    let epoch_date = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);

    match subcmd {
        // Epoch days are always statically 86400 seconds long.
        // Thus the following calculations are just "close enough" approximations
        YEARS => {
            println!("{}", (1970 - now.year()).abs());
        }
        MONTHS => {
            println!("{}", calculate_month_diff(epoch_date, now));
        }
        WEEKS => {
            let difference = epoch_date.signed_duration_since(now);
            println!("{}", difference.num_weeks().abs())
        }
        // ...and these naive calculations should actually be 100% correct
        DAYS => println!("{}", epoch / 60 / 60 / 24),
        HOURS => println!("{}", epoch / 60 / 60),
        MINUTES => println!("{}", epoch / 60),
        _ => println!("{}", epoch),
    }
}

fn handle_args(subcmd: &str, matches: &ArgMatches) {
    let now = Local::now();

    let from: DateTime<Local> = match matches.value_of("from") {
        Some(arg) => match try_parse_all_formats(arg, now) {
            Ok(datetime) => datetime,
            Err(err) => {
                eprintln!("Unable to parse FROM arg `{}` into datetime: {}.", arg, err);
                process::exit(1);
            }
        },
        None => {
            print_formatted_epoch(subcmd, now);
            process::exit(0);
        }
    };

    let to: DateTime<Local> = match matches.value_of("to") {
        Some(arg) => match try_parse_all_formats(arg, now) {
            Ok(datetime) => datetime,
            Err(err) => {
                eprintln!("Unable to parse TO arg `{}` into datetime: {}.", arg, err);
                process::exit(1);
            }
        },
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
Missing <to> argument will always default to current datetime.
All subcommands share exactly the same functionality and arguments as base
command, just filtering the output to different format.

All values are generally rounded down.";

    let matches = App::new("since")
        .about(about)
        .version("v0.11.0")
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(&from)
        .arg(&to)
        .subcommand(
            SubCommand::with_name(YEARS)
                .about("Print the output in years (approx)")
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
