# rec

[sqlite](https://www.sqlite.org/index.html)-backed timeseries recording and retrieval tool

Configuration: set `REC_DB_LOCATION` variable to the absolute path of the sqlite3 file you want to use; defaults to current directory.

Usage:

```bash
Using database at /home/gabe/src/rec/rec.sqlite3
Rec 0.1.0
Gabe Fierro

USAGE:
    rec <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add      add data to collections
    all      Get all data for the requested collections
    help     Prints this message or the help of the given subcommand(s)
    list     list collections
    query    query the data
    tadd     add data to collections with times
```

Examples:
```bash
cd /tmp
export REC_DB_LOCATION=rec.sqlite3
# add single values with current time
rec add testcollection 3
rec add testcollection 4
rec add testcollection 5
rec all testcollection
# => output
# test,3,2018-07-18T01:32:42.671418520+00:00
# test,4,2018-07-18T01:32:43.878220607+00:00
# test,5,2018-07-18T01:32:44.939063907+00:00

# filter > < >= <= == and or
rec query testcollection '> 3 and < 5'
# => output
# testcollection,4,2018-07-18T01:32:43.878220607+00:00

# add multiple values with same timestamp (now)
rec col1 3.4 col2 4.5
rec all
# => output
# col1
# col2

# add data with rfc3339 time
rec tadd x 3 2000-01-01T00:00:00Z
```
