#[derive(Default)]
pub struct FieldSpecParserBuilder {
    start_index: Option<usize>,
    complement: Option<bool>,
    range_separator: Option<String>,
    interval_separator: Option<String>,
}

impl FieldSpecParserBuilder {
    pub fn with_start_index(mut self, index: usize) -> Self {
        self.start_index = Some(index);
        self
    }

    pub fn inverse_match(mut self, complement: bool) -> Self {
        self.complement = Some(complement);
        self
    }

    pub fn with_range_separator(mut self, range_separator: String) -> Self {
        self.range_separator = Some(range_separator);
        self
    }

    pub fn with_interval_separator(mut self, interval_separator: String) -> Self {
        self.interval_separator = Some(interval_separator);
        self
    }

    pub fn build(self) -> FieldSpecParser {
        let start_index = self.start_index.unwrap_or_default();
        let complement = self.complement.unwrap_or_default();
        let range_separator = self.range_separator.unwrap_or_else(|| "-".to_string());
        let interval_separator = self.interval_separator.unwrap_or_else(|| ",".to_string());
        FieldSpecParser {
            fields: Vec::new(),
            open: false,
            mask: Vec::new(),
            start_index,
            complement,
            range_separator,
            interval_separator,
        }
    }
}

#[derive(Debug)]
pub struct FieldSpecParser {
    fields: Vec<usize>,
    open: bool,
    mask: Vec<bool>,
    start_index: usize,
    complement: bool,
    range_separator: String,
    interval_separator: String,
}

impl Default for FieldSpecParser {
    fn default() -> Self {
        FieldSpecParser::builder().build()
    }
}

impl FieldSpecParser {
    pub fn builder() -> FieldSpecParserBuilder {
        Default::default()
    }

    pub fn from_spec(&mut self, field_spec: &str) -> Result<(), &str> {
        let mut spec = String::new();
        if field_spec.starts_with(&self.range_separator) {
            spec.push_str(self.start_index.to_string().as_str());
        }
        spec.push_str(field_spec);
        if spec.ends_with(&self.range_separator) {
            self.open = true;
            spec.push_str(usize::max_value().to_string().as_str()); // Setting usize::MAX as a sentinel value
        }

        self.fields = spec
            .split(&self.interval_separator)
            .filter(|s| !s.is_empty())
            .flat_map(|interval| {
                let interval = interval
                    .split(&self.range_separator)
                    .map(|num| num.parse().unwrap_or_default())
                    .collect::<Vec<usize>>();
                match interval.len() {
                    1 => interval,
                    2 => {
                        if interval[1] == usize::max_value() {
                            vec![interval[0]]
                        } else {
                            (interval[0]..=interval[1]).collect()
                        }
                    }
                    _ => vec![0],
                }
            })
            .collect::<Vec<_>>();

        self.fields.sort();
        self.fields.dedup();

        if self.fields.first().is_none() || self.fields.first().unwrap() < &self.start_index {
            return Err("Invalid field specification");
        }

        let last = *self.fields.last().unwrap();
        self.mask = (self.start_index..=last)
            .map(|n| self.fields.binary_search(&n).is_ok() ^ self.complement)
            .collect();

        Ok(())
    }

    pub fn valid(&self, col: usize) -> bool {
        let col = col - self.start_index;
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
