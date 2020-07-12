use std::collections::BTreeSet;

#[derive(Debug)]
pub struct FieldSpecParser {
    fields: Vec<usize>,
    open: bool,
    complement: bool,
    mask: Vec<bool>,
}

impl FieldSpecParser {
    pub fn from_spec(field_spec: &str, complement: bool) -> Result<Self, &str> {
        let mut open = false;

        let mut spec = String::new();
        if field_spec.starts_with('-') {
            spec.push_str("1");
        }
        spec.push_str(&field_spec);
        if spec.ends_with('-') {
            open = true;
            spec.push_str(usize::max_value().to_string().as_str());
        }

        let fields = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|interval| {
                let interval = interval
                    .split('-')
                    .map(|num| num.parse().unwrap_or_default())
                    .collect::<Vec<usize>>();
                let items = match interval.len() {
                    1 => interval,
                    2 => {
                        if interval[1] == usize::max_value() {
                            vec![interval[0]]
                        } else {
                            (interval[0]..=interval[1]).collect()
                        }
                    }
                    _ => Vec::with_capacity(0),
                };
                if items.is_empty() {
                    vec![0]
                } else {
                    items
                }
            })
            .flatten()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        if fields.first().is_none() || fields.first().unwrap() == &0 {
            return Err("Invalid field specification");
        }

        let last = *fields.last().unwrap();
        let mask = (1..=last)
            .map(|n| fields.binary_search(&n).is_ok() ^ complement)
            .collect();
        Ok(FieldSpecParser {
            fields,
            open,
            complement,
            mask,
        })
    }

    #[allow(dead_code)]
    pub fn valid(&self, col: usize) -> bool {
        let col = col - 1;
        self.complement
            ^ if let Some(value) = self.mask.get(col) {
                *value
            } else {
                self.open
            }
    }

    pub fn mask_iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.mask
            .iter()
            .copied()
            .chain(std::iter::repeat(self.open ^ self.complement))
    }

    pub fn into_mask_iter(self) -> impl Iterator<Item = bool> {
        self.mask
            .into_iter()
            .chain(std::iter::repeat(self.open ^ self.complement))
    }
}
