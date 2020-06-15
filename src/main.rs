mod parsers;
mod subcommands;

#[cfg(test)]
mod tests;

use crate::parsers::try_parse_all_formats;
use crate::subcommands::Filter;
use chrono::{DateTime, Datelike, Duration, Local, TimeZone};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::process;

fn calculate_month_diff(from: DateTime<Local>, to: DateTime<Local>) -> i64 {
    // Individual typecasting is necessary to
    // a) compile at all
    // b) not panic from subtraction overflow
    let from_month = from.month() as i32;
    let to_month = to.month() as i32;

    // TODO: Figure out if we want a more precise formula here.
    ((from.year() - to.year()) * 12 + from_month - to_month).abs() as i64
}

fn calculate_year_diff(from: DateTime<Local>, to: DateTime<Local>) -> i64 {
    // TODO: Figure out if we want a more precise formula here.
    (from.year() - to.year()).abs() as i64
}

/// Print the time difference for shorthand use.
///
/// Does some basic guessing on which format is the nicest for user to read.
fn get_shorthand_output(
    from: DateTime<Local>,
    to: DateTime<Local>,
    difference: Duration,
) -> String {
    // TODO: Clean this up and add "and"-clauses to every option, like in hours/minutes now.
    let days = difference.num_days().abs();
    match days {
        // About two months to about two years
        63..=730 => format!("{} months", calculate_month_diff(from, to)),
        // Two days to about two months
        2..=62 => format!("{} days", difference.num_days().abs()),
        0..=1 => {
            let hours = difference.num_hours().abs();
            let mins = (difference.num_minutes() % 60).abs();

            // TODO: Seconds
            if hours == 0 {
                format!("{} minutes", mins)
            } else {
                format!("{} hours and {} minutes", hours, mins)
            }
        }
        _ => format!("{} years", calculate_year_diff(from, to)),
    }
}

/// Return the requested time difference filtered according to the chosen subcommand.
///
/// If no subcommand is chosen, guess which is the best format for humans to read
/// for the given time range.
fn get_output(from: DateTime<Local>, to: DateTime<Local>, filter: Filter) -> String {
    let difference = to.signed_duration_since(from);

    // NOTE:
    // All values are printed in absolutes, as to not show negative number for values in
    // future. While this is breaking the semantics of `since` a bit, we'll allow it for
    // better usability. You could basically just symlink `since` -> `until`.
    match filter {
        Filter::Years => calculate_year_diff(from, to).to_string(),
        Filter::Months => calculate_month_diff(from, to).to_string(),
        Filter::Weeks => difference.num_weeks().abs().to_string(),
        Filter::Days => difference.num_days().abs().to_string(),
        Filter::Hours => difference.num_hours().abs().to_string(),
        Filter::Minutes => difference.num_minutes().abs().to_string(),
        Filter::Seconds => difference.num_seconds().abs().to_string(),
        Filter::None => get_shorthand_output(from, to, difference),
    }
}

/// Return the UNIX timestamp filtered according to the chosen subcommand.
fn get_epoch_output(now: DateTime<Local>, filter: Filter) -> String {
    let epoch = Local::now().timestamp();
    let epoch_date = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let output: i64 = match filter {
        // Epoch days are always statically 86400 seconds long.
        // Thus the following calculations are just "close enough" approximations
        Filter::Years => (1970 - now.year()).abs() as i64,
        Filter::Months => calculate_month_diff(epoch_date, now),
        Filter::Weeks => {
            let difference = epoch_date.signed_duration_since(now);
            difference.num_weeks().abs()
        }
        // ...and these naive calculations should actually be 100% correct
        Filter::Days => epoch / 60 / 60 / 24,
        Filter::Hours => epoch / 60 / 60,
        Filter::Minutes => epoch / 60,
        Filter::Seconds => epoch,
        Filter::None => epoch,
    };

    output.to_string()
}

fn handle_args(filter: Filter, matches: &ArgMatches) {
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
            println!("{}", get_epoch_output(now, filter));
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

    println!("{}", get_output(from, to, filter));
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
            SubCommand::with_name(Filter::Years.as_str())
                .about("Print the output in years (approx)")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(Filter::Months.as_str())
                .about("Print the output in months (approx)")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(Filter::Weeks.as_str())
                .about("Print the output in weeks (approx)")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(Filter::Days.as_str())
                .about("Print the output in days")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(Filter::Hours.as_str())
                .about("Print the output in hours")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(Filter::Minutes.as_str())
                .about("Print the output in minutes")
                .arg(&from)
                .arg(&to),
        )
        .subcommand(
            SubCommand::with_name(Filter::Seconds.as_str())
                .about("Print the output in seconds")
                .arg(&from)
                .arg(&to),
        )
        .get_matches();

    match matches.subcommand() {
        (subcmd, Some(sub_matches)) => handle_args(Filter::from_str(subcmd), sub_matches),
        _ => handle_args(Filter::None, &matches),
    };
}
