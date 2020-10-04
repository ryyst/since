use chrono::{DateTime as DT, Datelike, Local, TimeZone, Timelike};

pub fn dt(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> DT<Local> {
    Local.ymd(y, m, d).and_hms(h, min, s)
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || (year % 100 == 0 && year % 400 == 0))
}

fn get_year_months(year: i32) -> Vec<i32> {
    let feb = if is_leap_year(year) { 29 } else { 28 };

    vec![31, feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

fn get_month_size(month: i32, year: i32) -> i32 {
    get_year_months(year)[(month - 1) as usize]
}

fn days_in_a_year(year: i32) -> i32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

// A year is the largest unit we have, so there's no differentiation between
// a fraction or a total value.
pub fn num_years(from: DT<Local>, to: DT<Local>) -> i64 {
    let years = (from.year() - to.year()).abs() as i64;

    if year_is_partial(from, to) {
        return years - 1;
    }

    years
}

//
// Total calculations for subcommand use. For example:
// - 428 days
// - 32 months
// - 42180 minutes

pub fn num_months_total(from: DT<Local>, to: DT<Local>) -> i64 {
    // Individual typecasting is necessary to
    // a) compile at all
    // b) not panic from subtraction overflow
    let from_month = from.month() as i32;
    let to_month = to.month() as i32;

    let mut value = ((from.year() - to.year()) * 12 + from_month - to_month).abs() as i64;

    if from.day() > to.day() {
        value -= 1;
    }

    value
}

pub fn num_weeks_total(from: DT<Local>, to: DT<Local>) -> i64 {
    to.signed_duration_since(from).num_weeks().abs()
}

pub fn num_days_total(from: DT<Local>, to: DT<Local>) -> i64 {
    to.signed_duration_since(from).num_days().abs()
}

pub fn num_hours_total(from: DT<Local>, to: DT<Local>) -> i64 {
    to.signed_duration_since(from).num_hours().abs()
}

pub fn num_minutes_total(from: DT<Local>, to: DT<Local>) -> i64 {
    to.signed_duration_since(from).num_minutes().abs()
}

pub fn num_seconds_total(from: DT<Local>, to: DT<Local>) -> i64 {
    to.signed_duration_since(from).num_seconds().abs()
}

//
// Partial comparisons for calulating difference fractions

fn minute_is_partial(from: DT<Local>, to: DT<Local>) -> bool {
    to.minute() == from.minute() && to.second() < from.second()
}

fn hour_is_partial(from: DT<Local>, to: DT<Local>) -> bool {
    to.hour() == from.hour() && to.minute() < from.minute() || minute_is_partial(from, to)
}

fn day_is_partial(from: DT<Local>, to: DT<Local>) -> bool {
    to.day() == from.day() && to.hour() < from.hour() || hour_is_partial(from, to)
}

fn month_is_partial(from: DT<Local>, to: DT<Local>) -> bool {
    //to.month() == from.month() &&
    to.day() < from.day() || day_is_partial(from, to)
}

fn year_is_partial(from: DT<Local>, to: DT<Local>) -> bool {
    to.month() < from.month() || month_is_partial(from, to)
}

//
// Fraction calculations for shorthand use. A fraction number must always be
// smaller than the next bigger unit. For example:
// - 28 days, 23 hours, 12 minutes
// - 42 years, 11 months

pub fn num_months_fraction(from: DT<Local>, to: DT<Local>) -> i64 {
    let to_year = to.year() as i32;
    let from_year = from.year() as i32;

    let to_month = to.month() as i32;
    let from_month = from.month() as i32;

    let goes_over_new_year = (to_year - from_year).abs() == 1;

    let months = if goes_over_new_year {
        12 - (to_month - from_month).abs() as i64
    } else {
        (to_month - from_month).abs() as i64
    };

    let mut months = num_months_total(from, to);

    if month_is_partial(from, to) {
        println!("Are we partial!?");
        months -= 1;
    }

    months % 12
}

fn days_passed_in_year(from: DT<Local>, to: DT<Local>) -> i64 {
    let from_month_sizes = get_year_months(from.year());
    let to_month_sizes = get_year_months(to.year());

    // For indexing with the size vectors
    let from_month = (from.month() - 1) as usize;
    let to_month = (to.month() - 1) as usize;

    let from_sum = if from_month == 11 {
        0
    } else {
        from_month_sizes[from_month..11].iter().sum()
    };

    let to_sum = if to_month == 0 {
        0
    } else {
        to_month_sizes[0..to_month].iter().sum()
    };

    // Which year to use for the elapsed year difference
    let year = if from_month > 1 {
        // From is already past february, ignore it
        to.year()
    } else {
        from.year()
    };

    let result = num_days_total(to, from) - (from_sum + to_sum) as i64;

    (result - 1) as i64
}

pub fn num_days_fraction(from: DT<Local>, to: DT<Local>) -> i64 {
    let from_year = from.year() as i32;
    let to_year = to.year() as i32;

    let from_month = from.month() as i32;
    let to_month = to.month() as i32;

    let to_day = to.day() as i32;
    let from_day = from.day() as i32;

    let goes_over_month = (to_month - from_month).abs() == 1 || to_month - from_month == -11;
    let goes_over_new_year = (to_year - from_year).abs() == 1 && goes_over_month;

    let from_month_size = get_month_size(from_month, from_year);
    //let _to_month_size = get_month_size(to_month, to_year);

    //let algo = ((from_month_size - from_day) + to_day) as i64;
    //println!(
    //    "Total days {} and from_month_size {}, months total {}",
    //    num_days_total(from, to),
    //    from_month_size,
    //    num_months_fraction(from, to)
    //);
    let days = if goes_over_month || goes_over_new_year {
        //println!("Getting from algo");
        num_days_total(from, to) % (from_month_size as i64)
    } else if (days_in_a_year(from.year())) as i64 - num_days_total(from, to) == 1 {
        // Inside the last day of a year difference
        // sum all days of the year and minus from total

        //println!("Getting from monthfraction");
        days_passed_in_year(from, to)
    } else if num_days_total(from, to) > from_month_size as i64 {
        //println!("Getting from elseif");
        (to_day - from_day).abs() as i64
    } else {
        //println!("Getting from else");
        num_days_total(from, to) as i64
    };

    //if day_is_partial(from, to) && month_is_partial(from, to) {
    //    from_month_size as i64 - 1
    //} else {
    days
    //}
}

pub fn num_hours_fraction(from: DT<Local>, to: DT<Local>) -> i64 {
    let to_day = to.day() as i32;
    let from_day = from.day() as i32;

    let to_hour = to.hour() as i32;
    let from_hour = from.hour() as i32;

    let goes_over_midnight = (to_day - from_day).abs() == 1;

    let hours = if goes_over_midnight {
        24 - (to_hour - from_hour).abs() as i64
    } else {
        (to_hour - from_hour).abs() as i64
    };

    if hour_is_partial(from, to) {
        return if hours - 1 < 0 { 23 } else { hours - 1 };
    }

    hours % 24
}

pub fn num_minutes_fraction(from: DT<Local>, to: DT<Local>) -> i64 {
    // Amazingly this just works
    num_minutes_total(from, to) % 60
}

pub fn num_seconds_fraction(from: DT<Local>, to: DT<Local>) -> i64 {
    num_seconds_total(from, to) % 60
}
