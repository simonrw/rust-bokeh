use super::errors::Result;
use super::glyphs::Glyph;
use super::idgen::create_id;
use super::layout::Layout;
use super::to_bokehjs::ToBokehJs;
use super::tools::Tool;
use super::ColumnDataSource;
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

        Ok(json!({
            "attributes": {
                "below": below_axes?,
                "left": left_axes?,
            }
        }))
    }

    fn id(&self) -> String {
        format!("{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::super::axes::LinearAxis;
    use super::super::to_bokehjs::compare_json;
    use super::super::tools::{PanTool, WheelZoomTool};
    use super::*;

    #[test]
    fn test_serialisation_includes_layouts() {
        let mut plot = Plot::with_id(1002);
        plot.add_layout(LinearAxis::with_id(1006), "below");
        plot.add_layout(LinearAxis::with_id(1007), "left");
        plot.add_tool(PanTool::new());
        plot.add_tool(WheelZoomTool::new());

        let json = plot.to_json().unwrap();
        compare_json(
            &json["attributes"]["below"],
            r##"[{"id": "1006", "type": "LinearAxis"}]"##,
        );
        compare_json(
            &json["attributes"]["left"],
            r##"[{"id": "1007", "type": "LinearAxis"}]"##,
        );
    }
}
