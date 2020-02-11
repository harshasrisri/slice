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
    files: Vec<PathBuf>,
}

fn main() {
    let args = CrustOpts::from_args();
    println!("{:?}", args);
}
