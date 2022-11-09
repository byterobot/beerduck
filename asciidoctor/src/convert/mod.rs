use std::fs;
use std::path::Path;
use std::sync::Mutex;

use anyhow::{anyhow, Error};
use deno_core::{Extension, JsRuntime, RuntimeOptions, serde_v8, v8};
use deno_core::error::AnyError;
use deno_core::op;
use once_cell::sync::Lazy;
use serde_json::Value;

static TEXT: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

#[op]
fn op_adoc() -> Result<String, AnyError> {
    let mut text = TEXT.lock().unwrap();
    match text.is_some() {
        true => Ok(text.take().unwrap()),
        _ => Err(AnyError::msg("TEXT is empty")),
    }
}

fn set_text(path: &Path) -> Result<(), Error> {
    TEXT.lock().unwrap().replace(fs::read_to_string(path)?);
    Ok(())
}

pub async fn convert(path: &Path) -> Result<String, Error> {
    set_text(path)?;
    let ext = Extension::builder().ops(vec![op_adoc::decl(),]).build();
    let options = RuntimeOptions { extensions: vec![ext], ..Default::default()};
    let mut runtime = JsRuntime::new(options);
    Ok(eval(&mut runtime, include_str!("js/convert.min.js.txt"))?)
}

fn eval(context: &mut JsRuntime, code: &str) -> Result<String, Error> {
    let res = context.execute_script("<anon>", code);
    match res {
        Ok(global) => {
            let scope = &mut context.handle_scope();
            let local = v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            match serde_v8::from_v8::<Value>(scope, local) {
                Ok(value) => match value {
                    Value::String(v) => Ok(v),
                    _ => Err(anyhow!("{}", value)),
                },
                Err(err) => Err(anyhow!("Cannot deserialize value: {:?}", err)),
            }
        }
        Err(err) => Err(anyhow!("Evaling error: {:?}", err)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use futures_await_test::async_test;

    use crate::convert::convert;

    #[async_test]
    async fn test() {
        let path = "";
        let a = convert(&Path::new(path)).await.unwrap();
        println!("{}", a);
    }
}
