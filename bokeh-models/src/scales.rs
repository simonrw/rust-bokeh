use crate::errors::Result;
use crate::idgen::create_id;
use crate::to_bokehjs::ToBokehJs;
use serde_json::Value;

#[derive(Default)]
pub struct LinearScale {
    id: i32,
}

impl LinearScale {
    pub fn new() -> Self {
        LinearScale::with_id(create_id())
    }

    fn with_id(id: i32) -> Self {
        LinearScale { id }
    }
}

pub(crate) trait Scale: ToBokehJs {}

impl Scale for LinearScale {}

impl ToBokehJs for LinearScale {
    fn to_json(&self) -> Result<Value> {
        Ok(json!({
            "id": self.id(),
            "type": "LinearScale",
        }))
    }

    fn id(&self) -> String {
        format!("{}", self.id)
    }
}
