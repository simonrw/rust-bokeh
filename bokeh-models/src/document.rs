use super::plot::Root;
use super::to_bokehjs::ToBokehJs;
use serde_json::Value;

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

impl ToBokehJs for Document {
    fn to_json(&self) -> Value {
        let root_ids = match self.root {
            Some(ref root) => vec![format!("{}", root.id())],
            None => unimplemented!(),
        };
        let references: Vec<Value> = vec![];

        json!({
            "roots": {
                "root_ids": root_ids,
                "references": references,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::plot::Plot;
    use super::*;

    #[test]
    fn test_document_root_ids() {
        let plot = Plot::with_id(1002);
        let mut doc = Document::new();
        doc.add_root(plot);

        let json = doc.to_json();
        let root_ids_node = &json["roots"]["root_ids"];

        assert!(root_ids_node.is_array());
        let root_ids = root_ids_node.as_array().unwrap();
        assert_eq!(root_ids.as_slice(), &["1002"]);
    }

    #[test]
    fn test_roots_have_references() {
        let plot = Plot::with_id(1002);
        let mut doc = Document::new();
        doc.add_root(plot);

        let json = doc.to_json();
        let roots_node = &json["roots"];

        assert!(
            roots_node["references"].is_array(),
            "{:#}",
            roots_node["references"]
        );
    }
}
