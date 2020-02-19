use std::collections::HashSet;

#[derive(Debug)]
pub struct FieldParser {
    fields: Vec<usize>,
    open: bool,
}

impl FieldParser {
    pub fn from_spec(field_spec: &str) -> Result<Self, &str> {
        let mut parser = FieldParser {
            fields: Vec::new(),
            open: false,
        };
        let mut spec_err = false;
        let error = Err("Invalid field specification");

        if field_spec.contains(",-")
            || field_spec.contains("-,")
            || field_spec.contains("--")
            || field_spec
                .chars()
                .any(|c| !c.is_numeric() && c != '-' && c != ',')
        {
            return error;
        }

        let mut spec = String::new();
        if field_spec.starts_with('-') {
            spec.push_str("1");
        }
        spec.push_str(&field_spec);
        while spec.ends_with('-') {
            parser.open = true;
            spec.pop();
        }

        parser.fields = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|interval| {
                if interval.starts_with('-') || interval.ends_with('-') {
                    spec_err = true;
                    return Vec::with_capacity(0);
                }
                let interval = interval
                    .split('-')
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<usize>>();
                if interval.len() == 1 {
                    interval
                } else if interval.len() == 2 {
                    (interval[0]..=interval[1]).collect()
                } else {
                    spec_err = true;
                    return Vec::with_capacity(0);
                }
            })
            .flatten()
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect();

        if spec_err {
            error
        } else {
            parser.fields.sort();
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
        res
    }
}
