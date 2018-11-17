pub trait Tool {}

#[derive(Clone)]
pub struct PanTool;

impl PanTool {
    pub fn new() -> PanTool {
        PanTool {}
    }
}

#[derive(Clone)]
pub struct WheelZoomTool;

impl WheelZoomTool {
    pub fn new() -> WheelZoomTool {
        WheelZoomTool {}
    }
}

impl Tool for PanTool {}
impl Tool for WheelZoomTool {}
