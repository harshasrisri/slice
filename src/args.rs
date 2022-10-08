use std::char::ParseCharError;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about,
    after_help = "
FIELD SPECIFICATION:
    The required fileds to be extracted can be specified or combined like below:
        3           => Extract column 3
        4-7         => Extract fields 4,5,6,7
        -5          => Extract all fields upto and including 5, i.e 1,2,3,4,5
        6-          => Extract all fields from and including 6, ie. 6,7,8,...
        2,4,6       => Extract only fields 2, 4 and 6
        -2,5-7,9-   => Extract fields 1,2,5,6,7,9,...
"
)]
pub struct SliceOpts {
    /// Fields to be extracted. See FIELD SPECIFICATION
    #[structopt(short, long, allow_hyphen_values = true)]
    pub fields: String,

    /// Start index of field separation
    #[structopt(short = "S", long, default_value = "0")]
    pub start_index: usize,

    /// Interval  separator in field specification
    #[structopt(short, long, default_value = ",")]
    pub interval_separator: String,

    /// Range  separator in field specification
    #[structopt(short, long, default_value = "-")]
    pub range_separator: String,

    /// Rows to be extracted. All, by default. See FIELD SPECIFICATION
    #[structopt(short, long, allow_hyphen_values = true)]
    pub rows: Option<String>,

    /// Delimiter to be used to split fields
    #[structopt(short, long, parse(try_from_str = parse_for_tab), default_value = " ")]
    pub delimiter: char,

    /// Separator to use to print results
    #[structopt(short, long, default_value = " ")]
    pub separator: String,

    /// Include lines that don't contain a delimiter
    #[structopt(short, long)]
    pub non_delimited: bool,

    /// Complement field spec. Print all fields but those specified with -f
    #[structopt(short, long)]
    pub complement: bool,

    /// Files to process
    #[structopt(name = "FILES", parse(from_os_str), default_value = "-")]
    pub files: Vec<PathBuf>,
}

fn parse_for_tab(src: &str) -> Result<char, ParseCharError> {
    if src == "\\t" {
        Ok('\t')
    } else {
        char::from_str(src)
    }
}
