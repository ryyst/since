extern crate chrono;

use chrono::{Datelike, Local, NaiveTime, TimeZone, Timelike};
use std::env;
use std::process;

struct Params {
    hours: u32,
    mins: u32,
}

impl Params {
    fn get() -> Params {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            println!("Not enough arguments");
            process::exit(1);
        }

        let hours_mins: NaiveTime = match NaiveTime::parse_from_str(&args[1], "%H:%M") {
            Ok(val) => val,
            Err(_err) => {
                println!("error goddammit, input something in the format of HH:MM");
                process::exit(1);
            }
        };
        let hours: u32 = hours_mins.hour();
        let mins: u32 = hours_mins.minute();

        Params { hours, mins }
    }
}

fn print_time(params: Params) {
    let now = Local::now();

    let target_time =
        Local
            .ymd(now.year(), now.month(), now.day())
            .and_hms(params.hours, params.mins, 0);

    let difference = now.signed_duration_since(target_time);

    if difference.num_seconds() < 0 {
        println!("No future values allowed here.");
        process::exit(1);
    }

    //let start = Local.datetime_from_str("7:35", "%M:%S").unwrap();
    let hours = difference.num_hours();
    let mins = difference.num_minutes() % 60;

    println!("{}h {}m", hours, mins);
}

fn main() {
    let params = Params::get();

    print_time(params);
}
