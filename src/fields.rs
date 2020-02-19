use std::collections::HashSet;

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

        let partially_parsed = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|interval| {
                if interval.starts_with('-') || interval.ends_with('-') {
                    return None;
                }
                let interval = interval
                    .split('-')
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<usize>>();
                if interval.len() == 1 {
                    Some(interval)
                } else if interval.len() == 2 {
                    Some((interval[0]..=interval[1]).collect())
                } else {
                    None
                }
            }).collect::<Vec<_>>();

        if partially_parsed.iter().any(|item| item.is_none()) {
            return error;
        }

        parser.fields = partially_parsed.into_iter()
            .flat_map(|item| item.unwrap())
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect();

        parser.fields.sort();
        Ok(parser)
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
