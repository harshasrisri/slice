use crate::FieldParser;

pub struct Splitter<'a> {
    fields: &'a FieldParser,
    delimiter: char,
    separator: &'a str,
}

impl<'a> Splitter<'a> {
    pub fn new(fields: &'a FieldParser, delimiter: char, separator: &'a str) -> Self {
        Splitter { fields, delimiter, separator }
    }

    pub fn parse(&self, line: &str) -> String {
        line.split(self.delimiter)
            .filter(|&s| s != "")
            .enumerate()
            .filter_map(|(i, word)| {
                if self.fields.valid_field(i + 1) {
                    Some(word)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(&self.separator)
    }
}
