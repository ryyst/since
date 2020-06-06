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
    years      Print the output in years
    months     Print the output in months (approx)
    weeks      Print the output in weeks (approx)
    days       Print the output in days
    hours      Print the output in hours
    minutes    Print the output in minutes
    seconds    Print the output in seconds
```
