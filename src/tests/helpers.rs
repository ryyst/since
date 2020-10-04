use chrono::{DateTime, Datelike, Local, TimeZone};

//
// A couple of shorthands, implemented with the same basic logic as the parsers.
//
pub fn dt(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> DateTime<Local> {
    Local.ymd(y, m, d).and_hms(h, min, s)
}

pub fn start_of_day(y: i32, m: u32, d: u32) -> DateTime<Local> {
    Local.ymd(y, m, d).and_hms(0, 0, 0)
}

pub fn time_today(h: u32, m: u32, s: u32, now: DateTime<Local>) -> DateTime<Local> {
    Local
        .ymd(now.year(), now.month(), now.day())
        .and_hms(h, m, s)
}
