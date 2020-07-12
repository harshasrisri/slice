use crate::fields::FieldSpecParser;
use std::fmt::Write;

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
