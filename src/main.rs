use std::fs::File;
use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

mod fields;
use fields::FieldParser;

mod args;
use args::CrustOpts;

mod split;
use split::Splitter;

fn main() {
    let args = CrustOpts::from_args();
    let parser = FieldParser::from_spec(&args.fields).expect("Error parsing fields");
    let splitter = Splitter::new(&parser, args.delimiter, args.separator);

    for file in args.files {
        let reader: Box<dyn BufRead> = match file.to_str() {
            Some("-") => Box::new(BufReader::new(io::stdin())),
            Some(path) => {
                if !file.exists() && !file.is_file() {
                    eprintln!("Invalid file: {}", file.to_str().unwrap_or_default());
                    std::process::exit(1);
                }

                Box::new(BufReader::new(File::open(path).unwrap()))
            }
            None => {
                eprintln!("Bad PathBuf");
                std::process::exit(1);
            }
        };

        for line in reader.lines().filter_map(|line| line.ok()) {
            println!("{}", splitter.parse(&line));
        }
    }
}
