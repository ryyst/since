use crate::calculators::{
    num_days_fraction, num_days_total, num_hours_fraction, num_hours_total, num_minutes_fraction,
    num_minutes_total, num_months_fraction, num_months_total, num_seconds_fraction,
    num_seconds_total, num_weeks_total, num_years,
};
use crate::subcommands::Filter;
use chrono::{DateTime, Local, TimeZone};
use std::cmp::min;

/// Return the requested time difference filtered according to the chosen subcommand.
///
/// If no subcommand is chosen, guess which is the best format for humans to read
/// for the given time range.
pub fn get_output(mut from: DateTime<Local>, mut to: DateTime<Local>, filter: Filter) -> String {
    if from > to {
        // Make sure from is always smaller. Generally the order doesn't matter,
        // but this just makes our own calculations simpler.
        std::mem::swap(&mut from, &mut to);
    }

    match filter {
        Filter::Years => num_years(from, to).to_string(),
        Filter::Months => num_months_total(from, to).to_string(),
        Filter::Weeks => num_weeks_total(from, to).to_string(),
        Filter::Days => num_days_total(from, to).to_string(),
        Filter::Hours => num_hours_total(from, to).to_string(),
        Filter::Minutes => num_minutes_total(from, to).to_string(),
        Filter::Seconds => num_seconds_total(from, to).to_string(),
        Filter::None => get_shorthand_output(from, to),
    }
}

/// Guess a good format to return the output in, roughly by the size of the difference.
fn get_shorthand_output(from: DateTime<Local>, to: DateTime<Local>) -> String {
    let mut output: Vec<String> = Vec::new();

    let years = num_years(from, to);
    let months = num_months_fraction(from, to);
    let days = num_days_fraction(from, to);
    let hours = num_hours_fraction(from, to);
    let mins = num_minutes_fraction(from, to);
    let seconds = num_seconds_fraction(from, to);

    if years > 0 {
        output.push(format!("{} years", years));
    }

    if months > 0 {
        output.push(format!("{} months", months));
    }

    if days > 0 {
        output.push(format!("{} days", days));
    }

    if hours > 0 {
        output.push(format!("{} hours", hours));
    }

    if mins > 0 {
        output.push(format!("{} minutes", mins));
    }

    if seconds > 0 || output.len() == 0 {
        output.push(format!("{} seconds", seconds));
    }

    // Show *at most* only the 3 most significant pieces of info
    let precision = min(output.len(), 3);
    output[..precision].join(", ")
}

/// Return the UNIX timestamp filtered according to the chosen subcommand.
pub fn get_epoch_output(now: DateTime<Local>, filter: Filter) -> String {
    let epoch = Local::now().timestamp();
    let epoch_date = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let output: i64 = match filter {
        // Epoch days are always statically 86400 seconds long.
        // Thus the following calculations are just "close enough" approximations
        Filter::Years => num_years(epoch_date, now),
        Filter::Months => num_months_total(epoch_date, now),
        // ...and these naive calculations should actually be 100% correct
        Filter::Weeks => epoch / 60 / 60 / 24 / 7,
        Filter::Days => epoch / 60 / 60 / 24,
        Filter::Hours => epoch / 60 / 60,
        Filter::Minutes => epoch / 60,
        Filter::Seconds => epoch,
        Filter::None => epoch,
    };

    output.to_string()
}
