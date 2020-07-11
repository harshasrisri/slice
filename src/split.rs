use crate::fields::FieldSpecParser;
use std::fmt::Write;

#[derive(Debug)]
pub struct Splitter {
    fields: FieldSpecParser,
    delimiter: char,
    separator: char,
}

impl Splitter {
    pub fn new(fields: FieldSpecParser, delimiter: char, separator: char) -> Self {
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
        let mut slices = input
            .split(self.delimiter)
            .filter(|&s| !s.is_empty())
            .zip(self.fields.mask_iter())
            .filter_map(|(field, allow)| if allow { Some(field) } else { None });
        if let Some(first) = slices.nth(0) {
            write!(&mut output, "{}", first)?;
        }
        for slice in slices {
            write!(&mut output, "{}{}", self.separator, slice)?;
        }
        Ok(())
    }
}
