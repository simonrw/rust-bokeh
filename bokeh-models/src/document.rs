use super::plot::Root;

pub struct Document;

impl Document {
    pub fn new() -> Document {
        Document {}
    }

    pub fn add_root<R>(&mut self, _root: &R)
    where
        R: Root,
    {
    }
}
