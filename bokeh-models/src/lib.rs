use serde_derive::Serialize;
use serde_json::{json, Value};

pub trait ToBokeh: serde::Serialize {
    fn to_bokeh_value(self) -> Value;
}

#[derive(Serialize)]
pub struct BasicTickFormatter;

impl BasicTickFormatter {
    pub fn new() -> BasicTickFormatter {
        BasicTickFormatter {}
    }
}

impl ToBokeh for BasicTickFormatter {
    fn to_bokeh_value(self) -> Value {
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
        let json_value: Value = tf.to_bokeh_value();
        assert_without_id_equal!(
            json_value,
            json!({
                "attributes": {},
                "type": "BasicTickFormatter",
            })
        );
    }
}
