#![allow(non_upper_case_globals)]

use crate::formatters::get_output;
use crate::subcommands::Filter;
use crate::tests::helpers::dt;

use chrono::{DateTime as DT, Local};

// Static values for easier time eyeballing the tens of eerily similar test
// values; any datetime differences are highlighted by not using these values.
static Y: i32 = 2019;
static M: u32 = 12;
static D: u32 = 24;

static h: u32 = 20;
static m: u32 = 15;
static s: u32 = 30;

// Note: All values are double-checked with an external tool, which was deemed
// to follow to the same principles of "intuitiveness" as we want for `since`.
//
// https://www.timeanddate.com/date/durationresult.html

#[test]
/// Test all precisions in order.
fn test_shorthand_output_basic_precisions_are_correct() {
    // Hardcoded "now". Makes sense, right?
    let now = dt(Y, M, D, h, m, s);

    // Smallest unit we'll display.
    assert_eq!("0 seconds", get_output(now, now, Filter::None));

    assert_eq!(
        "15 minutes, 30 seconds",
        get_output(dt(Y, M, D, h, 0, 0), now, Filter::None)
    );

    assert_eq!(
        "2 hours, 15 minutes, 30 seconds",
        get_output(dt(Y, M, D, 18, 0, 0), now, Filter::None)
    );

    assert_eq!(
        "3 days, 2 hours, 15 minutes",
        get_output(dt(Y, M, 21, 18, 0, 0), now, Filter::None)
    );

    assert_eq!(
        "4 months, 3 days, 2 hours",
        get_output(dt(Y, 8, 21, 18, 15, 30), now, Filter::None)
    );

    assert_eq!(
        "5 years, 4 months, 3 days",
        get_output(dt(2014, 8, 21, 18, 0, 0), now, Filter::None)
    );
}

#[test]
/// Mix and match varying precisions.
fn test_shorthand_output_varying_precisions_are_correct() {
    let now = dt(Y, M, D, h, m, s);

    // Missing days and minutes
    assert_eq!(
        "4 months, 2 hours, 30 seconds",
        get_output(dt(Y, 8, D, 18, m, 0), now, Filter::None)
    );

    // Missing everything in the middle
    assert_eq!(
        "5 years, 30 seconds",
        get_output(dt(2014, M, D, h, m, 0), now, Filter::None)
    );

    // Missing days
    assert_eq!(
        "4 months, 2 hours",
        get_output(dt(Y, 8, D, 18, m, s), now, Filter::None)
    );
}

#[test]
fn test_shorthand_output_year_boundary_is_correct() {
    let now = dt(Y, M, D, h, m, s);

    assert_eq!(
        "1 years, 1 seconds",
        get_output(dt(2018, M, D, h, m, 29), now, Filter::None)
    );

    assert_eq!(
        "11 months, 29 days, 23 hours", // + 59 minutes, 59 seconds
        get_output(dt(2018, M, D, h, m, 31), now, Filter::None)
    );
}

#[test]
fn test_shorthand_output_month_boundary_is_correct() {
    let now = dt(Y, M, D, h, m, s);

    // One second over a month, over a new year
    assert_eq!(
        "1 months, 1 seconds",
        get_output(dt(2020, 1, D, h, m, 31), now, Filter::None)
    );

    // One second less than a month, over a new year
    assert_eq!(
        "30 days, 23 hours, 59 minutes", // + 59 seconds
        get_output(dt(2020, 1, D, h, m, 29), now, Filter::None)
    );

    // Two days more than a month
    assert_eq!(
        "1 months, 2 days",
        get_output(dt(Y, 11, 22, h, m, s), now, Filter::None)
    );

    // Two days less than a month
    assert_eq!(
        "28 days",
        get_output(dt(Y, 11, 26, h, m, s), now, Filter::None)
    );

    // Playing around the edges
    assert_eq!(
        "29 days",
        get_output(
            dt(2019, 1, 31, h, m, s),
            dt(2019, 3, 1, h, m, s),
            Filter::None
        )
    );

    //assert_eq!(
    //    "1 months, 4 days",
    //    get_output(
    //        dt(2019, 2, 28, h, m, s),
    //        dt(2019, 4, 1, h, m, s),
    //        Filter::None
    //    )
    //);
}

/// Utility for testing shorthand output
fn test(output: &str, from: DT<Local>, to: DT<Local>) {
    assert_eq!(output, get_output(from, to, Filter::None));
}

#[test]
fn test_shorthand_output_day_boundary_is_correct() {
    let now = dt(Y, M, D, h, m, s);

    assert_eq!(
        // Since yesterday, one second more than 24h
        "1 days, 1 seconds",
        get_output(dt(Y, M, 23, h, m, 29), now, Filter::None)
    );

    assert_eq!(
        // Since yesterday, one second less than 24h
        "23 hours, 59 minutes, 59 seconds",
        get_output(dt(Y, M, 23, h, m, 31), now, Filter::None)
    );
}

#[test]
fn test_shorthand_output_hour_boundary_is_correct() {
    let now = dt(Y, M, D, h, m, s);

    // One second less than an hour
    assert_eq!(
        "59 minutes, 59 seconds",
        get_output(dt(Y, M, D, 21, m, 29), now, Filter::None)
    );

    // One second over an hour
    assert_eq!(
        "1 hours, 1 seconds",
        get_output(dt(Y, M, D, 21, m, 31), now, Filter::None)
    );

    // Extra: same, but in the past. Shouldn't matter at all.
    assert_eq!(
        "59 minutes, 59 seconds",
        get_output(dt(Y, M, D, 19, m, 31), now, Filter::None)
    );

    assert_eq!(
        "1 hours, 1 seconds",
        get_output(dt(Y, M, D, 19, m, 29), now, Filter::None)
    );
}

#[test]
fn test_shorthand_output_minute_boundary_is_correct() {
    let now = dt(Y, M, D, h, m, s);

    // One second less than an hour
    assert_eq!(
        "59 seconds",
        get_output(dt(Y, M, D, h, 16, 29), now, Filter::None)
    );

    // One second over an hour
    assert_eq!(
        "1 minutes, 1 seconds",
        get_output(dt(Y, M, D, h, 16, 31), now, Filter::None)
    );
}

#[test]
fn test_month_filter_output_looks_correct() {
    let now = dt(2019, 12, 24, 20, 15, 30);

    assert_eq!(
        "4",
        get_output(dt(2019, 8, 21, 18, 0, 0), now, Filter::Months)
    );
}
