use serde_json::{json, to_string, Result, Value};

pub trait ToBokeh {
    fn as_bokeh_value(&self) -> Value;
    fn as_string(&self) -> Result<String> {
        to_string(&self.as_bokeh_value())
    }
}

pub struct BasicTickFormatter;

impl BasicTickFormatter {
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
