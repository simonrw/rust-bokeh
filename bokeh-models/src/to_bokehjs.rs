use serde_json::Value;
#[cfg(test)]
use std::fmt::Display;

pub(crate) trait ToBokehJs {
    fn to_json(&self) -> Value;
}

#[cfg(test)]
pub(crate) trait ToValue {
    fn to_value(&self) -> Value;
}

#[cfg(test)]
impl ToValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }
}

#[cfg(test)]
impl<'a> ToValue for &'a Value {
    fn to_value(&self) -> Value {
        (*self).clone()
    }
}

#[cfg(test)]
impl<'a> ToValue for &'a str {
    fn to_value(&self) -> Value {
        serde_json::from_str(self).unwrap()
    }
}

#[cfg(test)]
impl ToValue for String {
    fn to_value(&self) -> Value {
        serde_json::from_str(self).unwrap()
    }
}

#[cfg(test)]
impl<'a> ToValue for &'a String {
    fn to_value(&self) -> Value {
        serde_json::from_str(self).unwrap()
    }
}

// Helper function to compare JSON strings
#[cfg(test)]
pub(crate) fn compare_json<A, B>(s1: A, s2: B)
where
    A: ToValue + Display,
    B: ToValue + Display,
{
    let v1: Value = s1.to_value();
    let v2: Value = s2.to_value();
    assert!(v1 == v2, "\n{:#}\nIS NOT EQUAL TO\n{:#}", v1, v2);
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
