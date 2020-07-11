use slice::args::SliceOpts;
use slice::fields::FieldSpecParser;
use slice::split::Splitter;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

fn main() {
    let args = SliceOpts::from_args();

    let splitter = if let Ok(parser) = FieldSpecParser::from_spec(&args.fields, args.complement) {
        Splitter::new(parser, args.delimiter, args.separator)
    } else {
        eprintln!("Failed to parse field spec");
        std::process::exit(1);
    };

    let mut output_line = String::new();
    let mut rows_iter: Box<dyn Iterator<Item = bool>> = if let Some(rows) = args.rows {
        if let Ok(row_spec) = FieldSpecParser::from_spec(&rows, false) {
            Box::new(row_spec.into_mask_iter())
        } else {
            eprintln!("Failed to parse row spec");
            std::process::exit(1);
        }
    } else {
        Box::new(std::iter::repeat(true))
    };

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
