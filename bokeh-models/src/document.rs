use crate::errors::Result;
use crate::idgen::create_id;
use crate::plot::Root;
use crate::scales::{LinearScale, Scale};
use crate::to_bokehjs::ToBokehJs;
use serde_json::Value;

#[derive(Debug)]
pub enum ValidationError {}

pub struct Document {
    id: i32,
    root: Option<Box<dyn Root>>,
    xscale: Box<dyn Scale>,
    yscale: Box<dyn Scale>,
}

impl Document {
    pub fn new() -> Document {
        let id = create_id();
        Self::with_id(id)
    }

    fn with_id(id: i32) -> Self {
        Document {
            id,
            root: None,
            xscale: Box::new(LinearScale::new()),
            yscale: Box::new(LinearScale::new()),
        }
    }

    pub fn add_root<R>(&mut self, root: R)
    where
        R: Root + 'static,
    {
        self.root = Some(Box::new(root));
    }

    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl ToBokehJs for Document {
    fn to_json(&self) -> Result<Value> {
        let root_ids = match self.root {
            Some(ref root) => vec![root.id()],
            None => unimplemented!(),
        };
        let references: Vec<Value> =
            vec![self.xscale.to_nested_json()?, self.yscale.to_nested_json()?];

        Ok(json!({
            "roots": {
                "root_ids": root_ids,
                "references": references,
            },
        }))
    }

    fn id(&self) -> String {
        format!("{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::Plot;

    #[test]
    fn test_document_root_ids() {
        let plot = Plot::with_id(1002);
        let mut doc = Document::new();
        doc.add_root(plot);

        let json = doc.to_json().unwrap();
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

        let json = doc.to_json().unwrap();
        let roots_node = &json["roots"];

        assert!(
            roots_node["references"].is_array(),
            "{:#}",
            roots_node["references"]
        );
    }

    #[test]
    fn test_document_outputs_linear_scales() {
        let plot = Plot::with_id(1002);
        let mut doc = Document::new();

        let x_id = doc.xscale.id();
        let y_id = doc.yscale.id();

        doc.add_root(plot);

        let json = doc.to_json().unwrap();
        let references = &json["roots"]["references"];
        println!("{}", json);
        assert!(references.is_array());
        let references = references.as_array().unwrap();

        let mut xfound = false;
        let mut yfound = false;
        for reference in references {
            assert!(reference.is_object());
            let ref_type = reference.get("type").as_ref().unwrap().as_str().unwrap();
            let ref_id = reference.get("id").as_ref().unwrap().as_str().unwrap();

            if ref_id == x_id {
                xfound = true;
                assert_eq!(ref_type, "LinearScale");
            } else if ref_id == y_id {
                yfound = true;
                assert_eq!(ref_type, "LinearScale");
            }
        }

        assert!(xfound && yfound);
    }
}
