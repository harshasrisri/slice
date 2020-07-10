use std::fs::File;
use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

mod fields;
use fields::FieldParser;

mod args;
use args::SliceOpts;

mod split;
use split::Splitter;

fn main() {
    let args = SliceOpts::from_args();
    let parser = FieldParser::from_spec(&args.fields, args.complement);
    if parser.is_err() {
        eprintln!("Failed to parse fields");
        std::process::exit(1);
    }
    let parser = parser.unwrap();
    let splitter = Splitter::new(&parser, args.delimiter, args.separator);
    let mut output_line = String::new();

    for file in args.files {
        let reader: Box<dyn BufRead> = match file.to_str() {
            Some("-") => Box::new(BufReader::new(io::stdin())),
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

        for line in reader.lines().filter_map(|line| line.ok()) {
            output_line.clear();
            splitter.parse_into(&line, &mut output_line).expect("");
            if output_line.is_empty() && !args.non_delimited {
                continue;
            }
            println!("{}", output_line);
        }
    }
}
