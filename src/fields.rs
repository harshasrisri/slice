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

        let partially_parsed = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .inspect(|item| println!("{:?}", item))
            .map(|interval| {
                if interval.starts_with('-') || interval.ends_with('-') {
                    return None;
                }
                let interval = interval
                    .split('-')
                    .map(|num| num.parse().unwrap_or_default())
                    .collect::<Vec<usize>>();
                if interval.len() == 1 {
                    Some(interval)
                } else if interval.len() == 2 {
                    Some((interval[0]..=interval[1]).collect())
                } else {
                    None
                }
            })
            .inspect(|item| println!("{:?}", item))
            .collect::<Vec<_>>();

        if partially_parsed
            .iter()
            .any(|item| item.is_none() || item.as_ref().unwrap().is_empty())
        {
            return error;
        }

        parser.fields = partially_parsed
            .into_iter()
            .flat_map(|item| item.unwrap())
            .collect::<BTreeSet<usize>>()
            .into_iter()
            .collect();

        if parser.fields.binary_search(&0).is_ok() {
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
