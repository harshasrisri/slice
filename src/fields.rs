use std::collections::BTreeSet;

#[derive(Debug)]
pub struct FieldParser {
    fields: Vec<usize>,
    open: bool,
    complement: bool,
}

impl FieldParser {
    pub fn from_spec(field_spec: &str, complement: bool) -> Result<Self, &str> {
        let mut parser = FieldParser {
            fields: Vec::new(),
            open: false,
            complement,
        };
        let error = Err("Invalid field specification");

        let mut spec = String::new();
        if field_spec.starts_with('-') {
            spec.push_str("1");
        }
        spec.push_str(&field_spec);
        if spec.ends_with('-') {
            parser.open = true;
            spec.pop();
        }

        parser.fields = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|interval| {
                let interval = interval
                    .split('-')
                    .map(|num| num.parse().unwrap_or_default())
                    .collect::<Vec<usize>>();
                let items = match interval.len() {
                    1 => interval,
                    2 => (interval[0]..=interval[1]).collect(),
                    _ => Vec::with_capacity(0),
                };
                if items.is_empty() {
                    vec![0]
                } else {
                    items
                }
            })
            .flatten()
            .collect::<BTreeSet<usize>>()
            .into_iter()
            .collect();

        if parser.fields[0] == 0 {
            error
        } else {
            Ok(parser)
        }
    }

    pub fn valid_field(&self, col: usize) -> bool {
        let mut res = self.fields.binary_search(&col).is_ok();
        if self.open {
            if let Some(last) = self.fields.last() {
                res = res || (col >= *last);
            }
        }
        res ^ self.complement
    }
}
