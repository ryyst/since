use crate::calculators::{calculate_month_diff, calculate_year_diff};
use crate::subcommands::Filter;
use chrono::{DateTime, Duration, Local, TimeZone};

/// Return the requested time difference filtered according to the chosen subcommand.
///
/// If no subcommand is chosen, guess which is the best format for humans to read
/// for the given time range.
pub fn get_output(from: DateTime<Local>, to: DateTime<Local>, filter: Filter) -> String {
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

/// Return the UNIX timestamp filtered according to the chosen subcommand.
pub fn get_epoch_output(now: DateTime<Local>, filter: Filter) -> String {
    let epoch = Local::now().timestamp();
    let epoch_date = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let output: i64 = match filter {
        // Epoch days are always statically 86400 seconds long.
        // Thus the following calculations are just "close enough" approximations
        Filter::Years => calculate_year_diff(epoch_date, now),
        Filter::Months => calculate_month_diff(epoch_date, now),
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
