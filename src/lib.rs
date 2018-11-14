use askama::Template;

// Rendering the output HTML
#[derive(Template)]
#[template(path = "index.html")]
struct PageTemplate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_template() {
        let page = PageTemplate {};
        let text = format!("{}", page.render().unwrap());
        assert!(text.contains("root.Bokeh.embed.embed_items"));
    }
}
