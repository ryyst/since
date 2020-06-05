extern crate chrono;
extern crate clap;

use chrono::{DateTime, Datelike, Local, NaiveTime, TimeZone, Timelike};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;

const DAYS: &str = "days";
const HOURS: &str = "hours";
const MINS: &str = "minutes";
const BASE: &str = "NOT_SUBCMD";

fn print_time_difference(from: DateTime<Local>, to: DateTime<Local>) {
    let difference = to.signed_duration_since(from);

    if difference.num_seconds() < 0 {
        println!("TODO: No future values allowed yet.");
        process::exit(1);
    }

    let hours = difference.num_hours();
    let mins = difference.num_minutes() % 60;

    println!("{:02}:{:02}", hours, mins);
}

fn fancy_print_epoch(cmd_name: &str) {
    let epoch = Local::now().timestamp();
    match cmd_name {
        DAYS => println!("{}", epoch / 60 / 60 / 24),
        HOURS => println!("{}", epoch / 60 / 60),
        MINS => println!("{}", epoch / 60),
        _ => println!("{}", epoch),
    }
}

fn try_parse_time(arg: &str, now: &DateTime<Local>) -> DateTime<Local> {
    match NaiveTime::parse_from_str(&arg, "%H:%M") {
        Ok(val) => {
            Local
                .ymd(now.year(), now.month(), now.day())
                .and_hms(val.hour(), val.minute(), 0)
        }
        Err(_err) => {
            println!("TODO: %Y-%M-%D");
            process::exit(1);
        }
    }
}

fn handle_args(name: &str, matches: &ArgMatches) {
    let now = Local::now();

    let from: DateTime<Local> = match matches.value_of("from") {
        Some(val) => try_parse_time(val, &now),
        None => {
            fancy_print_epoch(name);
            process::exit(1);
        }
    };

    let to: DateTime<Local> = match matches.value_of("to") {
        Some(val) => try_parse_time(val, &now),
        None => now,
    };

    print_time_difference(from, to);
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

    let matches = App::new("since")
        .about("Fetch time difference")
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
            SubCommand::with_name(MINS)
                .about("Return output in minutes")
                .arg(&from)
                .arg(&to),
        )
        .get_matches();

    match matches.subcommand() {
        (name, Some(sub_matches)) => handle_args(name, sub_matches),
        _ => handle_args(BASE, &matches),
    }
}
