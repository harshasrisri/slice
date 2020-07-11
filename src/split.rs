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

    pub fn parse<'b>(&self, line: &'b str) -> Vec<&'b str> {
        line.split(self.delimiter)
            .filter(|&s| !s.is_empty())
            .zip(self.fields.mask_iter())
            .filter_map(|(word, allow)| if allow { Some(word) } else { None })
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
