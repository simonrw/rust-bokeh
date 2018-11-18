use super::idgen::create_id;
use super::to_bokehjs::ToBokehJs;
use serde_json::Value;

pub trait Tool {}

#[derive(Clone)]
pub struct PanTool {
    id: i32,
}

impl PanTool {
    pub fn new() -> Self {
        let id = create_id();
        Self::with_id(id)
    }

    fn with_id(id: i32) -> Self {
        PanTool { id }
    }
}

#[derive(Clone)]
pub struct WheelZoomTool {
    id: i32,
}

impl WheelZoomTool {
    pub fn new() -> Self {
        let id = create_id();
        Self::with_id(id)
    }

    fn with_id(id: i32) -> Self {
        WheelZoomTool { id }
    }
}

impl ToBokehJs for WheelZoomTool {
    fn to_json(&self) -> Value {
        json!({
            "attributes": {},
            "id": format!("{}", self.id),
            "type": "WheelZoomTool",
        })
    }
}

impl ToBokehJs for PanTool {
    fn to_json(&self) -> Value {
        json!({
            "attributes": {},
            "id": format!("{}", self.id),
            "type": "PanTool",
        })
    }
}

impl Tool for PanTool {}
impl Tool for WheelZoomTool {}

#[cfg(test)]
mod tests {
    use super::super::to_bokehjs::compare_json;
    use super::*;

    #[test]
    fn test_wheel_zoom_tool_serialisation() {
        let wzt = WheelZoomTool::with_id(1009);

        let json = wzt.to_json();
        let expected = r##"{
            "attributes": {},
            "id": "1009",
            "type": "WheelZoomTool"
        }"##;

        compare_json(&json, expected);
    }

    #[test]
    fn test_pan_tool_serialisation() {
        let pt = PanTool::with_id(1008);

        let json = pt.to_json();
        let expected = r##"{
            "attributes": {},
            "id": "1008",
            "type": "PanTool"
        }"##;

        compare_json(&json, expected);
    }
}
