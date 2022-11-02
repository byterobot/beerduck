use std::collections::HashMap;

use serde_json::Value;
use tera::Tera;

pub fn register(tera: &mut Tera) {
    tera.register_tester("none", is_none);
    tera.register_function("unwrap", unwrap);
}

fn is_none(value: Option<&Value>, _b: &[Value]) -> tera::Result<bool> {
    match value {
        Some(Value::Null) | None => Ok(true),
        _ => Ok(false),
    }
}

pub fn unwrap(args: &HashMap<String, Value>) -> tera::Result<Value> {
    match args.get("value") {
        Some(v) => Ok(v.clone()),
        _ => Err(tera::Error::msg("Function `unwrap` didn't receive a `value` argument"))
    }
}