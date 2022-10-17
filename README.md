# Slice

I wrote this to scratch an itch that `cut` utility was a tad bit unintuitive. In the process, I got to address another pet peeve of mine that there is no easy tool to filter rows based on row numbers.

Not aiming to be performant(yet), although the performance is only slightly worse than `cut`. Not aiming to have feature parity with `cut`(yet), but just to have a close approximation with a different set of defaults.

## Default behavior
* Doesn't have a bytes/chars mode of operation
* Omits lines not containing a delimiter
* Uses space as the delimiter(IFS) and the separator(OFS)
* Trailing separators in input are not included in output
* Treats consecutive separators as one (not configurable at the moment)

## Usage

Some [examples](examples.md) are included.
<!-- `$ echo \$ slice --help && cargo run --quiet --release -- --help` -->
```
$ slice --help
An opinionated implementation of the 'cut' *nix utility to slice rows and columns of data.

Usage: slice [OPTIONS] --fields <FIELDS> [FILES]...

Arguments:
  [FILES]...  Files to process [default: -]

Options:
  -f, --fields <FIELDS>        Fields to be extracted. See FIELD SPECIFICATION
  -r, --rows <ROWS>            Rows to be extracted. All, by default. See FIELD SPECIFICATION
  -d, --delimiter <DELIMITER>  Delimiter to be used to split fields [default: " "]
  -s, --separator <SEPARATOR>  Separator to use to print results [default: " "]
  -n, --non-delimited          Include lines that don't contain a delimiter
  -c, --complement             Complement field spec. Print all fields but those specified with -f
  -h, --help                   Print help information
  -V, --version                Print version information


FIELD SPECIFICATION:
    The required fileds to be extracted can be specified or combined like below:
        3           => Extract column 3
        4-7         => Extract fields 4,5,6,7
        -5          => Extract all fields upto and including 5, i.e 1,2,3,4,5
        6-          => Extract all fields from and including 6, ie. 6,7,8,...
        2,4,6       => Extract only fields 2, 4 and 6
        -2,5-7,9-   => Extract fields 1,2,5,6,7,9,...
```
