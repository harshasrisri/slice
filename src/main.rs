use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(after_help = "
FIELD SPECIFICATION:
    The required fileds to be extracted can be specified or combined like below:
        3           => Extract column 3
        4-7         => Extract columns 4,5,6,7
        -5          => Extract all columns upto and including 5, i.e 1,2,3,4,5
        6-          => Extract all columns from and including 6, ie. 6,7,8,...
        2,4,6       => Extract only columns 2, 4 and 6
        -2,5-7,9-   => Extract columns 1,2,5,6,7,9,...
")]
pub struct CrustOpts {
    /// Specify the fields to be extracted.
    #[structopt(short, long)]
    pub field_spec: String,

    /// Specify the delimiter to be used to split columns
    #[structopt(short, long, default_value = "\t")]
    pub delimiter: char,

    /// Include lines that don't contain a delimiter
    #[structopt(short, long)]
    pub non_delimited: bool,

    /// Complement field spec. Print all columns but those specified with -f
    #[structopt(short, long)]
    pub complement: bool,

    /// Files to process
    #[structopt(name = "FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct Fields<'a> {
    field_spec: &'a str,
    fields: Vec<usize>,
    open: bool,
}

impl<'a> Fields<'a> {
    pub fn new(field_spec: &'a str) -> Self {
        let mut fields = Fields {
            field_spec,
            fields: Vec::new(),
            open: false,
        };
        fields.parse();
        fields
    }

    pub fn parse(&mut self) {
        let mut spec = String::new();
        if self.field_spec.starts_with('-') {
            spec.push_str("1");
        }
        spec.push_str(self.field_spec);
        while spec.ends_with('-') {
            self.open = true;
            spec.pop();
        }

        let spec_err = || {
            eprintln!("Invalid field specification: {}", self.field_spec);
            std::process::exit(1);
        };

        self.fields = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|interval| {
                if interval.starts_with('-') || interval.ends_with('-') {
                    spec_err();
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
                    spec_err()
                }
            })
            .flatten()
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect();

        self.fields.sort();
    }
}

fn main() {
    let args = CrustOpts::from_args();
    let fields = Fields::new(&args.field_spec);
    println!("{:?}", args);
    println!("{:?}", fields);
}
