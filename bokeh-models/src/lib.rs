//! Rust implementation of the client-side portion of Bokeh.

#![deny(missing_docs)]

use serde_json::{json, to_string, Result, Value};

/// Trait encoding the ability to transform the type into their Bokeh representation
pub trait ToBokeh {
    /// Compulsory method for converting Bokeh model into serializable JSON
    ///
    /// This must be implemented by any struct that is to be converted to Bokeh type, and sent to
    /// BokehJS in the browser
    fn as_bokeh_value(&self) -> Value;

    /// Convert a bokeh struct to string
    ///
    /// Automatically implemented for objects based on their `ToBokeh::as_bokeh_value`
    /// implementation.
    fn as_string(&self) -> Result<String> {
        to_string(&ToBokeh::as_bokeh_value(self))
    }
}

/// Struct dealing with basic tick formatting.
pub struct BasicTickFormatter;

impl BasicTickFormatter {
    /// Create a new BasicTickFormatter
    pub fn new() -> BasicTickFormatter {
        BasicTickFormatter {}
    }
}

impl ToBokeh for BasicTickFormatter {
    fn as_bokeh_value(&self) -> Value {
        json!({
            "attributes": {},
            "type": "BasicTickFormatter",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_without_id_equal {
        ($left:expr, $right:expr) => {
            assert!($left.is_object(), "Left value {} is not an object", $left);
            assert!(
                $right.is_object(),
                "Right value {} is not an object",
                $right
            );

            let lo = $left.as_object().unwrap();
            let ro = $right.as_object().unwrap().clone();

            for key in ro.keys() {
                assert!(lo.contains_key(key), "Key `{}` missing from output", key);

                let lvalue = &lo[key];
                let rvalue = &ro[key];

                assert_eq!(lvalue, rvalue);
            }
        };
    }

    #[test]
    fn test_basic_tick_formatter() {
        let tf = BasicTickFormatter::new();
        let json_value: Value = tf.as_bokeh_value();
        assert_without_id_equal!(
            json_value,
            json!({
                "attributes": {},
                "type": "BasicTickFormatter",
            })
        );
    }
}
