use crate::fields::FieldParser;
use std::fmt::Write;

#[derive(Debug)]
pub struct Splitter<'a> {
    fields: &'a FieldParser,
    delimiter: char,
    separator: char,
}

impl<'a> Splitter<'a> {
    pub fn new(fields: &'a FieldParser, delimiter: char, separator: char) -> Self {
        Splitter {
            fields,
            delimiter,
            separator,
        }
    }

    pub fn parse<'b>(&self, line: &'b str) -> Vec<&'b str> {
        line.split(self.delimiter)
            .filter(|&s| !s.is_empty())
            .enumerate()
            .filter_map(|(i, word)| {
                if self.fields.valid_field(i + 1) {
                    Some(word)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn parse_into<T>(&self, input: &str, mut output: &mut T) -> Result<(), std::fmt::Error>
    where
        T: Write,
    {
        let slices = self.parse(input);
        for slice in slices.iter().take(slices.len() - 1) {
            write!(&mut output, "{}{}", slice, self.separator)?;
        }
        if let Some(last) = slices.last() {
            write!(&mut output, "{}", last)?;
        }
        Ok(())
    }
}
