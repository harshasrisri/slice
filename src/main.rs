use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(after_help = "
FIELD SPECIFICATION:
    The required fileds to be extracted can be specified or combined like below:
        3           => Extract column 3
        4-7         => Extract fields 4,5,6,7
        -5          => Extract all fields upto and including 5, i.e 1,2,3,4,5
        6-          => Extract all fields from and including 6, ie. 6,7,8,...
        2,4,6       => Extract only fields 2, 4 and 6
        -2,5-7,9-   => Extract fields 1,2,5,6,7,9,...
")]
pub struct CrustOpts {
    /// Specify the fields to be extracted.
    #[structopt(short, long, allow_hyphen_values = true)]
    fields: String,

    /// Specify the delimiter to be used to split fields
    #[structopt(short, long, default_value = "\t")]
    delimiter: char,

    /// Include lines that don't contain a delimiter
    #[structopt(short, long)]
    non_delimited: bool,

    /// Complement field spec. Print all fields but those specified with -f
    #[structopt(short, long)]
    complement: bool,

    /// Files to process
    #[structopt(name = "FILES", parse(from_os_str))]
    files: Vec<PathBuf>,

    #[structopt(skip)]
    fields_list: Vec<usize>,

    #[structopt(skip)]
    open: bool,
}

impl CrustOpts {
    pub fn parse_fields(&mut self) {
        let spec_err = |spec| {
            eprintln!("Invalid field specification: {}", spec);
            std::process::exit(1);
        };

        if self.fields.contains(",-")
            || self.fields.contains("-,")
            || self.fields.contains("--")
            || self
                .fields
                .chars()
                .any(|c| !c.is_numeric() && c != '-' && c != ',')
        {
            spec_err(&self.fields);
        }

        let mut spec = String::new();
        if self.fields.starts_with('-') {
            spec.push_str("1");
        }
        spec.push_str(&self.fields);
        while spec.ends_with('-') {
            self.open = true;
            spec.pop();
        }

        self.fields_list = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|interval| {
                if interval.starts_with('-') || interval.ends_with('-') {
                    spec_err(&self.fields);
                }
                let interval = interval
                    .split('-')
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<usize>>();
                if interval.len() == 1 {
                    interval
                } else if interval.len() == 2 {
                    (interval[0]..=interval[1]).collect()
                } else {
                    spec_err(&self.fields)
                }
            })
            .flatten()
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect();

        self.fields_list.sort();
    }
}

fn main() {
    let mut args = CrustOpts::from_args();
    args.parse_fields();
    println!("{:?}", args);
}
