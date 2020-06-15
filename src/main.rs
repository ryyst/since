mod calculators;
mod formatters;
mod parsers;
mod subcommands;

#[cfg(test)]
mod tests;

use crate::formatters::{get_epoch_output, get_output};
use crate::parsers::try_parse_all_formats;
use crate::subcommands::Filter;
use chrono::{DateTime, Local};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::process;

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
