# Slice

Wrote as a project to scratch an itch that `cut` utility was a tad bit unintuitive. 

Not aiming to be performant(yet). Not aiming to have feature parity with `cut`(yet). Just to have a close approximation with a different set of defaults.

## Default behavior
* Doesn't have a bytes/chars mode of operation
* Omits lines not containing a delimiter
* Uses space as the delimiter(IFS) and the separator(OFS)
* Trailing separators in input are not included in output

## Usage

Some [examples](examples.md) are included.

```
slice 0.2.0
An opinionated implementation of the 'cut' *nix utility to slice rows and columns of data.

USAGE:
    slice [FLAGS] [OPTIONS] --fields <fields> [FILES]...

FLAGS:
    -c, --complement       Complement field spec. Print all fields but those specified with -f
    -h, --help             Prints help information
    -n, --non-delimited    Include lines that don't contain a delimiter
    -V, --version          Prints version information

OPTIONS:
    -d, --delimiter <delimiter>    Delimiter to be used to split fields [default:  ]
    -f, --fields <fields>          Fields to be extracted. See FIELD SPECIFICATION
    -r, --rows <rows>              Rows to be extracted. All, by default. See FIELD SPECIFICATION
    -s, --separator <separator>    Separator to use to print results [default:  ]

ARGS:
    <FILES>...    Files to process [default: -]


FIELD SPECIFICATION:
    The required fileds to be extracted can be specified or combined like below:
        3           => Extract column 3
        4-7         => Extract fields 4,5,6,7
        -5          => Extract all fields upto and including 5, i.e 1,2,3,4,5
        6-          => Extract all fields from and including 6, ie. 6,7,8,...
        2,4,6       => Extract only fields 2, 4 and 6
        -2,5-7,9-   => Extract fields 1,2,5,6,7,9,...
```
