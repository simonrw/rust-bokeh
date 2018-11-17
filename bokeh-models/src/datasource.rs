pub trait DataSource {}

impl<T> DataSource for Vec<T> {}

#[derive(Clone)]
pub struct ColumnDataSource;

impl ColumnDataSource {
    pub fn new() -> ColumnDataSource {
        ColumnDataSource {}
    }

    pub fn add<D>(&mut self, _name: &str, _source: &D)
    where
        D: DataSource + Clone,
    {
    }
}
