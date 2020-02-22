use crate::FieldParser;

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

    pub fn parse(&self, line: &str) -> String {
        let mut buf = [0; 4];
        let sep = self.separator.encode_utf8(&mut buf);
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
            .join(sep)
    }
}
