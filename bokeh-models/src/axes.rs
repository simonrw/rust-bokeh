use super::idgen::create_id;
use super::layout::Layout;
use super::to_bokehjs::ToBokehJs;
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
    fn to_json(&self) -> Value {
        json!({
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
            "id": format!("{}", self.id),
            "type": "LinearAxis",
        })
    }

    fn id(&self) -> i32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::super::to_bokehjs::compare_json;
    use super::*;

    #[test]
    fn test_linear_axis_serialisation() {
        let linear_axis = LinearAxis::with_id(1007);

        let json = linear_axis.to_json();
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

        compare_json(&json, expected);
    }
}
