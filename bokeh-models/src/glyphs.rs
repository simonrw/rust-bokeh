use crate::errors::Result;
use crate::idgen::create_id;
use crate::to_bokehjs::ToBokehJs;
use serde_json::Value;
use std::collections::HashMap;

pub trait Glyph {}

#[derive(Default, Clone)]
pub struct Circle {
    id: i32,
    pub x: Option<String>,
    pub y: Option<String>,
    pub fill_color: Option<String>,
    pub size: Option<u32>,
    pub line_color: Option<String>,
}

impl Circle {
    pub fn new() -> Circle {
        let id = create_id();
        Circle::with_id(id)
    }

    // Helper method for testing
    fn with_id(id: i32) -> Circle {
        let mut circle = Circle::default();
        circle.id = id;
        circle
    }
}

impl Glyph for Circle {}

impl ToBokehJs for Circle {
    fn to_json(&self) -> Result<Value> {
        let mut attributes = HashMap::new();

        if let Some(ref fill_color) = self.fill_color {
            attributes.insert("fill_color", json!({ "value": fill_color }));
        }

        if let Some(ref size) = self.size {
            attributes.insert("size", json!({ "units": "screen", "value": size }));
        }

        if let Some(ref x) = self.x {
            attributes.insert("x", json!({ "field": x }));
        }

        if let Some(ref y) = self.y {
            attributes.insert("y", json!({ "field": y }));
        }

        Ok(json!({
            "id": self.id(),
            "type": "Circle",
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
    use crate::to_bokehjs::compare_json;

    #[test]
    fn test_serialisation() {
        let mut circle = Circle::with_id(1003);
        circle.x = Some("x".to_string());
        circle.y = Some("y".to_string());
        circle.fill_color = Some("red".to_string());
        circle.size = Some(5);
        circle.line_color = Some("black".to_string());

        let json = circle.to_json().unwrap();
        let expected = r##"{ "attributes": { "fill_color": { "value": "red" }, "size": { "units": "screen", "value": 5 }, "x": { "field": "x" }, "y": { "field": "y" } }, "id": "1003", "type": "Circle" }"##;

        compare_json(&json, expected).unwrap();
    }
}
