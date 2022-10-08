use slice::args::SliceOpts;
use slice::fields::FieldSpecParser;
use slice::split::Splitter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

fn main() {
    let args = SliceOpts::from_args();

    let mut parser = FieldSpecParser::builder()
        .inverse_match(args.complement)
        .with_range_separator(args.range_separator)
        .with_interval_separator(args.interval_separator)
        .with_start_index(args.start_index)
        .finish();

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
