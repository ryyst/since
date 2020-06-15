use chrono::{DateTime, Datelike, Local};

pub fn calculate_month_diff(from: DateTime<Local>, to: DateTime<Local>) -> i64 {
    // Individual typecasting is necessary to
    // a) compile at all
    // b) not panic from subtraction overflow
    let from_month = from.month() as i32;
    let to_month = to.month() as i32;

    // TODO: Figure out if we want a more precise formula here.
    ((from.year() - to.year()) * 12 + from_month - to_month).abs() as i64
}

pub fn calculate_year_diff(from: DateTime<Local>, to: DateTime<Local>) -> i64 {
    // TODO: Figure out if we want a more precise formula here.
    (from.year() - to.year()).abs() as i64
}
