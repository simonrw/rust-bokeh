extern crate askama;

use askama::Template;

type Guid = String;

mod models {
    include!(concat!(env!("OUT_DIR"), "/models.rs"));
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // Test to render static content to a string
    #[test]
    fn test_manual_render_to_string() {
        let plot_data = r##"{"525559c6-ff05-4b07-a440-71d3780e6d1d":{"roots":{"references":[{"attributes":{"overlay":{"id":"1028","type":"BoxAnnotation"}},"id":"1022","type":"BoxZoomTool"},{"attributes":{"plot":{"id":"1001","subtype":"Figure","type":"Plot"},"ticker":{"id":"1011","type":"BasicTicker"}},"id":"1014","type":"Grid"},{"attributes":{},"id":"1023","type":"SaveTool"},{"attributes":{"plot":null,"text":""},"id":"1041","type":"Title"},{"attributes":{"formatter":{"id":"1044","type":"BasicTickFormatter"},"plot":{"id":"1001","subtype":"Figure","type":"Plot"},"ticker":{"id":"1016","type":"BasicTicker"}},"id":"1015","type":"LinearAxis"},{"attributes":{},"id":"1024","type":"ResetTool"},{"attributes":{"callback":null},"id":"1002","type":"DataRange1d"},{"attributes":{},"id":"1042","type":"BasicTickFormatter"},{"attributes":{},"id":"1016","type":"BasicTicker"},{"attributes":{},"id":"1025","type":"HelpTool"},{"attributes":{},"id":"1044","type":"BasicTickFormatter"},{"attributes":{"dimension":1,"plot":{"id":"1001","subtype":"Figure","type":"Plot"},"ticker":{"id":"1016","type":"BasicTicker"}},"id":"1019","type":"Grid"},{"attributes":{"callback":null},"id":"1004","type":"DataRange1d"},{"attributes":{"active_drag":"auto","active_inspect":"auto","active_multi":null,"active_scroll":"auto","active_tap":"auto","tools":[{"id":"1020","type":"PanTool"},{"id":"1021","type":"WheelZoomTool"},{"id":"1022","type":"BoxZoomTool"},{"id":"1023","type":"SaveTool"},{"id":"1024","type":"ResetTool"},{"id":"1025","type":"HelpTool"}]},"id":"1026","type":"Toolbar"},{"attributes":{"data_source":{"id":"1035","type":"ColumnDataSource"},"glyph":{"id":"1036","type":"Circle"},"hover_glyph":null,"muted_glyph":null,"nonselection_glyph":{"id":"1037","type":"Circle"},"selection_glyph":null,"view":{"id":"1039","type":"CDSView"}},"id":"1038","type":"GlyphRenderer"},{"attributes":{},"id":"1047","type":"Selection"},{"attributes":{},"id":"1048","type":"UnionRenderers"},{"attributes":{},"id":"1006","type":"LinearScale"},{"attributes":{"bottom_units":"screen","fill_alpha":{"value":0.5},"fill_color":{"value":"lightgrey"},"left_units":"screen","level":"overlay","line_alpha":{"value":1.0},"line_color":{"value":"black"},"line_dash":[4,4],"line_width":{"value":2},"plot":null,"render_mode":"css","right_units":"screen","top_units":"screen"},"id":"1028","type":"BoxAnnotation"},{"attributes":{"fill_color":{"value":"#1f77b4"},"line_color":{"value":"#1f77b4"},"x":{"field":"x"},"y":{"field":"y"}},"id":"1036","type":"Circle"},{"attributes":{},"id":"1008","type":"LinearScale"},{"attributes":{"fill_alpha":{"value":0.1},"fill_color":{"value":"#1f77b4"},"line_alpha":{"value":0.1},"line_color":{"value":"#1f77b4"},"x":{"field":"x"},"y":{"field":"y"}},"id":"1037","type":"Circle"},{"attributes":{"formatter":{"id":"1042","type":"BasicTickFormatter"},"plot":{"id":"1001","subtype":"Figure","type":"Plot"},"ticker":{"id":"1011","type":"BasicTicker"}},"id":"1010","type":"LinearAxis"},{"attributes":{"source":{"id":"1035","type":"ColumnDataSource"}},"id":"1039","type":"CDSView"},{"attributes":{},"id":"1020","type":"PanTool"},{"attributes":{"below":[{"id":"1010","type":"LinearAxis"}],"left":[{"id":"1015","type":"LinearAxis"}],"renderers":[{"id":"1010","type":"LinearAxis"},{"id":"1014","type":"Grid"},{"id":"1015","type":"LinearAxis"},{"id":"1019","type":"Grid"},{"id":"1028","type":"BoxAnnotation"},{"id":"1038","type":"GlyphRenderer"}],"title":{"id":"1041","type":"Title"},"toolbar":{"id":"1026","type":"Toolbar"},"x_range":{"id":"1002","type":"DataRange1d"},"x_scale":{"id":"1006","type":"LinearScale"},"y_range":{"id":"1004","type":"DataRange1d"},"y_scale":{"id":"1008","type":"LinearScale"}},"id":"1001","subtype":"Figure","type":"Plot"},{"attributes":{},"id":"1011","type":"BasicTicker"},{"attributes":{"callback":null,"data":{"x":[1,2,3],"y":[4,5,6]},"selected":{"id":"1047","type":"Selection"},"selection_policy":{"id":"1048","type":"UnionRenderers"}},"id":"1035","type":"ColumnDataSource"},{"attributes":{},"id":"1021","type":"WheelZoomTool"}],"root_ids":["1001"]},"title":"Bokeh Application","version":"1.0.1-10-g8691b77dfe95"}}"##;
        let page = PageTemplate {
            doc_id: "525559c6-ff05-4b07-a440-71d3780e6d1d".to_string(),
            placeholder_id: 1112,
            plot_id: 1001,
            plot_guid: "398f0a3d-51fc-4aff-9df6-e569ebbc486e".to_string(),
            plot_data: plot_data.to_string(),
        };
        let text = format!("{}", page.render().unwrap());
        // TODO: better check
        assert!(text.contains("root.Bokeh.embed.embed_items"));
    }
}
