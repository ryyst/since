# Since

Since what?

```
23:19:53 risto@GALACTICA /home/risto>
↪ since 15:00
08:20
```


## Semantic CLI utility for calculating time differences

```
↪ since -h
since v0.9

Fetch time difference between <from> and <to>.

If no parameters are given, will return time since UNIX epoch.
Missing <to> argument will always default to current date/time.

All values are generally rounded down.


USAGE:
    since [ARGS] [SUBCOMMAND]

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

- `HH:MM`
- `HH:MM:SS`
- `YYYY-MM-DD`
- `DD-MM-YYYY`


**Notes:**

- Date formats support separators `-`, `/` and `.`.
- `December` and `Dec` are also valid formats in place of month.
- In spite of holy UI semantics, future values are also supported. All values are always absolute
  *distances*, thus no negative values should ever appear.
- All output (especially weeks/months/years) should generally be treated as "intuitive approximations",
  due to ambiguity in the definition of `when has 1 $time_unit passed`.
- All calculations are done in the local system's timezone


## Examples

```
↪ date
Sun  7 Jun 10:25:58 EEST 2020

↪ since 9:00
01:26

↪ since hours 9:00
1

↪ since minutes 9:00
86

↪ since days 9:00
0

↪ since days 24.12.2019
165

↪ since months 24.12.2019
6

↪ since years 24.12.2002
18

↪ since
1591514892

↪ since minutes
26525248

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

- Format guessing for basemode
- Datetime format
- Tests
- Improve documentation
