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

    trait ToValue {
        fn to_value(&self) -> Value;
    }

    impl ToValue for Value {
        fn to_value(&self) -> Value {
            self.clone()
        }
    }

    impl<'a> ToValue for &'a Value {
        fn to_value(&self) -> Value {
            (*self).clone()
        }
    }

    impl<'a> ToValue for &'a str {
        fn to_value(&self) -> Value {
            serde_json::from_str(self).unwrap()
        }
    }

    impl ToValue for String {
        fn to_value(&self) -> Value {
            serde_json::from_str(self).unwrap()
        }
    }

    impl<'a> ToValue for &'a String {
        fn to_value(&self) -> Value {
            serde_json::from_str(self).unwrap()
        }
    }

    use std::fmt::Display;

    // Helper macro to compare JSON strings
    fn compare_json<A, B>(s1: A, s2: B)
    where
        A: ToValue + Display,
        B: ToValue + Display,
    {
        let v1: Value = s1.to_value();
        let v2: Value = s2.to_value();
        assert!(v1 == v2, "{} != {}", s1, s2);
    }

    #[test]
    fn test_basic_tick_formatter() {
        let t = BasicTickFormatter::default();

        let json = t.to_json();
        compare_json(&json, "{\"id\": \"1363\", \"js_event_callbacks\": {}, \"js_property_callbacks\": {}, \"name\": null, \"power_limit_high\": 5, \"power_limit_low\": -3, \"precision\": \"auto\", \"subscribed_events\": [], \"tags\": [], \"use_scientific\": true}");
    }
}
