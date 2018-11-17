use std::collections::HashMap;

pub trait DataSource {}

impl<T> DataSource for Vec<T> {}

pub struct ColumnDataSource {
    mapping: HashMap<String, Box<dyn DataSource>>,
}

impl ColumnDataSource {
    pub fn new() -> ColumnDataSource {
        ColumnDataSource {
            mapping: HashMap::new(),
        }
    }

    pub fn add<D>(&mut self, name: &str, source: D)
    where
        D: DataSource + 'static,
    {
        self.mapping.insert(name.to_string(), Box::new(source));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_things() {
        let x: Vec<_> = (0..100).collect();
        let mut source = ColumnDataSource::new();
        source.add("x", x);
    }
}
