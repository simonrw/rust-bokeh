use serde_json::Value;

pub(crate) trait ToBokehJs {
    fn to_json(&self) -> Value;
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
}
