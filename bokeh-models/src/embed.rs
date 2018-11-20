use crate::document::Document;
use crate::errors::Result;
use crate::to_bokehjs::ToBokehJs;
use failure::format_err;

pub fn file_html(document: &Document, title: &str) -> Result<String> {
    let mut json = document.to_json()?;

    // Add the title
    let obj = json
        .as_object_mut()
        .ok_or_else(|| format_err!("node is not an object"))?;
    obj.insert(String::from("title"), json!(title));

    Ok(format!("{}", json!(obj)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{document::Document, plot::Plot};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use serde_json::{self, Value};

    #[test]
    fn test_title() {
        let plot = Plot::with_id(1002);
        let mut doc = Document::new();
        doc.add_root(plot);

        let random_string: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();

        let html_string = file_html(&doc, &random_string).unwrap();
        let v: Value = serde_json::from_str(&html_string).unwrap();

        assert_eq!(v["title"], random_string);
    }
}
