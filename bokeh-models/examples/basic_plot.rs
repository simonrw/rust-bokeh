extern crate bokeh_models;

use bokeh_models::document::Document;
use bokeh_models::embed::file_html;
use bokeh_models::glyphs::Circle;
use bokeh_models::{ColumnDataSource, LinearAxis, PanTool, Plot, WheelZoomTool};
use std::f64::consts;
use std::fs::File;
use std::io::Write;

fn main() {
    // Build the data set
    let x: Vec<_> = (0..100)
        .map(|i| (4.0 * consts::PI / 100.0) * (i as f64) - (2.0 * consts::PI))
        .collect();
    let y: Vec<_> = x.iter().map(|xval| xval.sin()).collect();

    let mut source = ColumnDataSource::new();
    source.add("x", x);
    source.add("y", y);

    /* TODO
    let mut source = column_data_source! {
        "x" => x,
        "y" => y,
    };
    */

    let mut plot = Plot::new();
    plot.min_border = Some(80);

    let mut circle = Circle::new();
    circle.x = Some("x".to_string());
    circle.y = Some("y".to_string());
    circle.fill_color = Some("red".to_string());
    circle.size = Some(5);
    circle.line_color = Some("black".to_string());

    plot.add_glyph(&source, circle);

    plot.add_layout(LinearAxis::new(), "below");
    plot.add_layout(LinearAxis::new(), "left");

    plot.add_tool(PanTool::new());
    plot.add_tool(WheelZoomTool::new());

    let mut doc = Document::new();
    doc.add_root(plot);

    if let Err(e) = doc.validate() {
        panic!("Error validating plot: {:?}", e);
    }

    let filename = "/tmp/basic_plot.html";
    let mut f = File::create(filename).expect("creating output file");
    write!(f, "{}", file_html(&doc, "Basic Glyph Plot").unwrap()).expect("writing file contents");
    println!("Wrote {}", filename);
}
