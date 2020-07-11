use std::fs::File;
use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

mod fields;
use fields::FieldSpecParser;

mod args;
use args::SliceOpts;

mod split;
use split::Splitter;

fn main() {
    let args = SliceOpts::from_args();

    let splitter = if let Ok(parser) = FieldSpecParser::from_spec(&args.fields, args.complement) {
        Splitter::new(parser, args.delimiter, args.separator)
    } else {
        eprintln!("Failed to parse fields");
        std::process::exit(1);
    };

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
