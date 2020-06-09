# Since

Since what?

```
14:30:29 risto@GALACTICA /home/risto>
↪ since 13:00
1 hours and 30 minutes
```

Install with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```
cargo install since
```

Now you should have the binary in `~/.cargo/bin/since` (on Linux systems).


## Semantic CLI utility for calculating time differences

```
↪ since -h
since v0.11.0

Fetch time difference between <from> and <to>.

If no parameters are given, will return time since UNIX epoch.
Missing <to> argument will always default to current datetime.
All subcommands share exactly the same functionality and arguments as base
command, just filtering the output to different format.

All values are generally rounded down.

USAGE:
    since [ARGS]
    since <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <from>    Start time or date.
    <to>      End time or date, for custom range. Default is current datetime.

SUBCOMMANDS:
    years      Print the output in years (approx)
    months     Print the output in months (approx)
    weeks      Print the output in weeks (approx)
    days       Print the output in days
    hours      Print the output in hours
    minutes    Print the output in minutes
    seconds    Print the output in seconds
```


## Supported date & time formats

All formats are hard-coded and chosen with the loose criteria of "what makes sense", the goal is to be
intuitive, not exhaustive (yet).

### Time
- `HH:MM`
- `HH:MM:SS`

### Date
- `YYYY-MM-DD`
- `YYYY/MM/DD`
- `YYYY.MM.DD`
- `DD-MM-YYYY`
- `DD/MM/YYYY`
- `DD.MM.YYYY`
- `DD MONTH YYYY` (eg. Dec or December)
- `YYYY MONTH DD`

### Datetime
- `[date] [time]`
- `[date]T[time]`


## Notes on functionality

- In spite of holy UI semantics, future values are also supported. All values are always absolute
  *distances*, thus no negative values should ever appear.
- All output (especially weeks/months/years) should generally be treated as "intuitive approximations",
  due to ambiguity in the definition of `when has 1 $time_unit passed`.
- All calculations are done in the local system's timezone
- Space-separated parameters must *always* be wrapped in quotes
- All naive times (eg. 12:15) are always intepreted as "today"


## Examples

```sh
↪ date
Mon  8 Jun 10:12:14 EEST 2020

# The original usecase, shorthand time calculations
↪ since 7:00
3 hours and 12 minutes

↪ since 24.12.2019
6 months

↪ since 24-12-2012T16:00:00
8 years

# Explicit time output formats
↪ since hours 7:00
3

↪ since minutes 7:00
192

↪ since days 7:00
0

↪ since days 24.12.2019
166

↪ since months 24.12.2019
6

↪ since years 24.12.2002
18

# Custom time ranges with explicit second parameter
↪ since minutes 14:00 23:45
585

# Mixing parameter formats
↪ since hours "24 December 2019 18:15:30" 2020-6-20
4286

# Unix epochs
↪ since
1591600334

↪ since minutes
26526672

↪ since years
50
```


## Build

```
git clone https://github.com/ryyst/since.git
cd since
cargo build --release
```

Now you can find the binary in `./target/release/since`


## Todo

- Calculation & output tests
- Improve documentation
- More accurate year/month calculations
- A flag for decimal output
- Exhaustive time fractions for the "guessed" output
  - eg. "1 years, 2 months, 3 days, 5 hours and 15 minutes"
