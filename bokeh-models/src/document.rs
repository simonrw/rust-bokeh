use super::plot::Root;

#[derive(Debug)]
pub enum ValidationError {}

#[derive(Default)]
pub struct Document {
    root: Option<Box<dyn Root>>,
}

impl Document {
    pub fn new() -> Document {
        Document::default()
    }

    pub fn add_root<R>(&mut self, root: R)
    where
        R: Root + 'static,
    {
        self.root = Some(Box::new(root));
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}
