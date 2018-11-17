pub(crate) trait ToBokehJs {
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Null
    }
}
