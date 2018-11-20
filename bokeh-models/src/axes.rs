use crate::errors::Result;
use crate::idgen::create_id;
use crate::layout::Layout;
use crate::to_bokehjs::ToBokehJs;
use serde_json::Value;

#[derive(Clone)]
pub struct LinearAxis {
    id: i32,
}

impl LinearAxis {
    pub fn new() -> Self {
        let id = create_id();
        Self::with_id(id)
    }

    pub(crate) fn with_id(id: i32) -> Self {
        Self { id }
    }
}

impl Layout for LinearAxis {}

impl ToBokehJs for LinearAxis {
    fn to_json(&self) -> Result<Value> {
        Ok(json!({
            "attributes": {
                // TODO
                "formatter": {
                    "id": "1018",
                    "type": "BasicTickFormatter",
                },
                // TODO
                "plot": {
                    "id": "1002",
                    "type": "Plot",
                },
                // TODO
                "ticker": {
                    "id": "1019",
                    "type": "BasicTicker",
                },
            },
            "id": self.id(),
            "type": "LinearAxis",
        }))
    }

    fn to_nested_json(&self) -> Result<Value> {
        Ok(json!({
            "id": self.id(),
            "type": "LinearAxis",
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
    fn test_linear_axis_serialisation() {
        let linear_axis = LinearAxis::with_id(1007);

        let json = linear_axis.to_json().unwrap();
        let expected = r##"{
        "attributes": {
          "formatter": {
            "id": "1018",
            "type": "BasicTickFormatter"
          },
          "plot": {
            "id": "1002",
            "type": "Plot"
          },
          "ticker": {
            "id": "1019",
            "type": "BasicTicker"
          }
        },
        "id": "1007",
        "type": "LinearAxis"
      }"##;

        compare_json(&json, expected).unwrap();
    }
}
