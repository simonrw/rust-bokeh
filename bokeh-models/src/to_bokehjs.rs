use serde_json::Value;

pub(crate) trait ToBokehJs {
    fn to_json(&self) -> Value;
}

#[derive(Default)]
struct BasicTickFormatter;

impl ToBokehJs for BasicTickFormatter {
    fn to_json(&self) -> Value {
        json!({
            "id": "1363",
            "js_event_callbacks": {},
            "js_property_callbacks": {},
            "name": null,
            "power_limit_high": 5,
            "power_limit_low": -3,
            "precision": "auto",
            "subscribed_events": [],
            "tags": [],
            "use_scientific": true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_to_json() {
        /* Simple test to set up the mechanism and assert that the trait can
         * be applied to arbitrary structs */
        struct Foo;

        impl ToBokehJs for Foo {
            fn to_json(&self) -> Value {
                Value::Null
            }
        }

        let json = Foo.to_json();
        let json_str = serde_json::to_string(&json).unwrap();
        assert_eq!(json_str, "null");
    }

    fn compare_json(s1: &str, s2: &str) -> bool {
        let v1: Value = serde_json::from_str(s1).unwrap();
        let v2: Value = serde_json::from_str(s2).unwrap();
        v1 == v2
    }

    #[test]
    fn test_basic_tick_formatter() {
        let t = BasicTickFormatter::default();

        let json = t.to_json();
        let json_str = serde_json::to_string(&json).unwrap();
        assert!(compare_json(&json_str, "{\"id\": \"1363\", \"js_event_callbacks\": {}, \"js_property_callbacks\": {}, \"name\": null, \"power_limit_high\": 5, \"power_limit_low\": -3, \"precision\": \"auto\", \"subscribed_events\": [], \"tags\": [], \"use_scientific\": true}"));
    }
}
