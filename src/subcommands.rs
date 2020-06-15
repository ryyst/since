// While using enum-strings is over-verbose compared to simple const strings,
// we'll consider this a fun excercise if nothing else.

/// All supported subcommand branches
pub enum Filter {
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
    None,
}

impl Filter {
    // Convert enum to string value, for use with Clap.
    pub fn as_str(&self) -> &str {
        match self {
            Filter::Years => "years",
            Filter::Months => "months",
            Filter::Weeks => "weeks",
            Filter::Days => "days",
            Filter::Hours => "hours",
            Filter::Minutes => "minutes",
            Filter::Seconds => "seconds",
            Filter::None => "NOT_SUBCMD",
        }
    }

    // ...and back to enum from Clap.
    pub fn from_str(s: &str) -> Filter {
        match s {
            "years" => Filter::Years,
            "months" => Filter::Months,
            "weeks" => Filter::Weeks,
            "days" => Filter::Days,
            "hours" => Filter::Hours,
            "minutes" => Filter::Minutes,
            "seconds" => Filter::Seconds,
            _ => Filter::None,
        }
    }
}
