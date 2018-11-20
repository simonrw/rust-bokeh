use crate::errors::Result;
use crate::glyphs::Glyph;
use crate::idgen::create_id;
use crate::layout::Layout;
use crate::to_bokehjs::ToBokehJs;
use crate::tools::Tool;
use crate::ColumnDataSource;
use serde_json::Value;
use std::collections::HashMap;

pub trait Root: ToBokehJs {}

#[derive(Default)]
pub struct Plot {
    pub min_border: Option<i32>,
    glyphs: Vec<Box<dyn Glyph>>,
    layouts: HashMap<String, Box<dyn Layout>>,
    tools: Vec<Box<dyn Tool>>,
    id: i32,
}

impl Plot {
    pub fn new() -> Plot {
        let id = create_id();
        Plot::with_id(id)
    }

    pub(crate) fn with_id(id: i32) -> Self {
        let mut plot = Plot::default();
        plot.id = id;
        plot
    }

    pub fn add_glyph<G>(&mut self, _source: &ColumnDataSource, glyph: G)
    where
        G: Glyph + 'static,
    {
        self.glyphs.push(Box::new(glyph));
    }

    pub fn add_layout<L>(&mut self, layout: L, position: &str)
    where
        L: Layout + 'static,
    {
        self.layouts.insert(position.to_string(), Box::new(layout));
    }

    pub fn add_tool<T>(&mut self, tool: T)
    where
        T: Tool + 'static,
    {
        self.tools.push(Box::new(tool));
    }
}

impl Root for Plot {}

impl ToBokehJs for Plot {
    fn to_json(&self) -> Result<Value> {
        let below_axes: Result<Vec<_>> = self
            .layouts
            .iter()
            .filter(|(k, _)| *k == "below")
            .map(|(_, v)| v.to_nested_json())
            .collect();
        let left_axes: Result<Vec<_>> = self
            .layouts
            .iter()
            .filter(|(k, _)| *k == "left")
            .map(|(_, v)| v.to_nested_json())
            .collect();

        let mut attributes = HashMap::new();
        attributes.insert("below", json!(below_axes?));
        attributes.insert("left", json!(left_axes?));

        if let Some(min_border) = self.min_border {
            attributes.insert("min_border", json!(min_border));
        }

        Ok(json!({
            "attributes": attributes,
        }))
    }

    fn id(&self) -> String {
        format!("{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axes::LinearAxis;
    use crate::to_bokehjs::compare_json;
    use crate::tools::{PanTool, WheelZoomTool};

    fn create_example_plot() -> Plot {
        let mut plot = Plot::with_id(1002);
        plot.min_border = Some(80);
        plot.add_layout(LinearAxis::with_id(1006), "below");
        plot.add_layout(LinearAxis::with_id(1007), "left");
        plot.add_tool(PanTool::new());
        plot.add_tool(WheelZoomTool::new());
        plot
    }

    fn example_json() -> Value {
        create_example_plot().to_json().unwrap()
    }

    #[test]
    fn test_serialisation_includes_layouts() {
        let json = example_json();
        compare_json(
            &json["attributes"]["below"],
            r##"[{"id": "1006", "type": "LinearAxis"}]"##,
        );
        compare_json(
            &json["attributes"]["left"],
            r##"[{"id": "1007", "type": "LinearAxis"}]"##,
        );
    }

    #[test]
    fn test_serialisation_includes_min_border() {
        let json = example_json();
        compare_json(&json["attributes"]["min_border"], json!(80));
    }
}
