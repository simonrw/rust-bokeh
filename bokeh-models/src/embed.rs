use askama::Template;
use crate::document::Document;
use crate::errors::Result;
use crate::to_bokehjs::ToBokehJs;
use failure::format_err;
use serde_json::Value;

type Guid = String;

// Rendering the output HTML
#[derive(Template)]
#[template(path = "index.html")]
struct PageTemplate {
    doc_id: Guid,
    placeholder_id: u64,
    plot_data: String,
    plot_id: u64,
    plot_guid: Guid,
}

fn render_html_and_embed_json(doc: Value) -> Result<String> {
    let page_template = PageTemplate {
        doc_id: "525559c6-ff05-4b07-a440-71d3780e6d1d".to_string(),
        placeholder_id: 1112,
        plot_id: 1001,
        plot_guid: "398f0a3d-51fc-4aff-9df6-e569ebbc486e".to_string(),
        plot_data: format!("{}", doc),
    };

    Ok(format!("{}", page_template.render()?))
}

pub fn file_html(document: &Document, title: &str) -> Result<String> {
    let mut json = document.to_json()?;

    // Add the title
    let obj = json
        .as_object_mut()
        .ok_or_else(|| format_err!("node is not an object"))?;
    obj.insert(String::from("title"), json!(title));

    render_html_and_embed_json(json!(obj))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{document::Document, plot::Plot};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_title() {
        let plot = Plot::with_id(1002);
        let mut doc = Document::new();
        doc.add_root(plot);

        let random_string: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();

        let html_string = file_html(&doc, &random_string).unwrap();
        assert!(html_string.contains(&random_string));
    }
}
