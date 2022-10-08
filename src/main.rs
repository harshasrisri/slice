use field_spec::FieldSpecParser;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

fn parse_for_tab(src: &str) -> Result<char, std::char::ParseCharError> {
    if src == "\\t" {
        Ok('\t')
    } else {
        char::from_str(src)
    }
}

#[derive(Debug)]
pub struct Splitter {
    fields: FieldSpecParser,
    delimiter: char,
    separator: String,
}

impl Splitter {
    pub fn new(fields: FieldSpecParser, delimiter: char, separator: String) -> Self {
        Splitter {
            fields,
            delimiter,
            separator,
        }
    }

    pub fn parse_into<T>(&self, input: &str, mut output: &mut T) -> Result<(), std::fmt::Error>
    where
        T: Write,
    {
        write!(
            &mut output,
            "{}",
            input
                .split(self.delimiter)
                .filter(|&s| !s.is_empty())
                .zip(self.fields.mask_iter())
                .filter_map(|(field, allow)| if allow { Some(field) } else { None })
                .collect::<Vec<_>>()
                .join(&self.separator)
        )
    }
}

fn main() {
    let args = SliceOpts::from_args();

    let mut parser = FieldSpecParser::builder()
        .inverse_match(args.complement)
        .with_range_separator(args.range_separator)
        .with_interval_separator(args.interval_separator)
        .with_start_index(args.start_index)
        .build();

    let splitter = if let Err(e) = parser.from_spec(&args.fields) {
        eprintln!("Failed to parse field spec: {}", e);
        std::process::exit(1);
    } else {
        Splitter::new(parser, args.delimiter, args.separator)
    };

    let mut rows_iter: Box<dyn Iterator<Item = bool>> = if let Some(rows) = args.rows {
        let mut row_parser: FieldSpecParser = Default::default();
        if let Err(e) = row_parser.from_spec(&rows) {
            eprintln!("Failed to parse row spec: {}", e);
            std::process::exit(1);
        } else {
            Box::new(row_parser.into_mask_iter())
        }
    } else {
        Box::new(std::iter::repeat(true))
    };

    let mut output_line = String::new();
    for file in args.files {
        let reader: Box<dyn BufRead> = match file.to_str() {
            Some("-") => Box::new(BufReader::new(std::io::stdin())),
            Some(_) => {
                if !file.exists() && !file.is_file() {
                    eprintln!("Invalid file: {}", file.to_str().unwrap_or_default());
                    std::process::exit(1);
                }
                Box::new(BufReader::new(File::open(file).unwrap()))
            }
            None => {
                eprintln!("Bad filename - {}", file.display());
                std::process::exit(1);
            }
        };

        for line in rows_iter
            .by_ref()
            .zip(reader.lines())
            .filter_map(|(use_line, line)| if use_line { Some(line) } else { None })
        {
            let line = line.expect("Error reading input");
            output_line.clear();
            splitter
                .parse_into(&line, &mut output_line)
                .expect("Error parsing input");
            if output_line.is_empty() && !args.non_delimited {
                continue;
            }
            println!("{}", output_line);
        }
    }
}
