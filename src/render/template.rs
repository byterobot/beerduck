use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Error};
use minify_html_onepass::Cfg;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;
use tera::{Context, Function, Tera};

use crate::config::CONFIG;

static TERA: Lazy<Tera> = Lazy::new(|| {
    let dir = CONFIG.workspace.templates.join("*.html");
    let dir = dir.to_str().expect("Invalid directory for templates");
    let mut tera = Tera::new(dir).expect("new tera error");
    tera.autoescape_on(Vec::new());
    tera.register_tester("none", is_none);
    tera.register_function("unwrap", unwrap);
    tera
});

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


pub enum Template {
    Article, Category, Categories, Index, About,
}

impl Template {
    pub fn render_write(&self, value: impl Serialize, target: &Path) -> Result<(), Error> {
        let template_name = match self {
            Template::Article => "article.html",
            Template::Category => "category.html",
            Template::Categories => "categories.html",
            Template::Index => "index.html",
            Template::About => "about.html"
        };

        let mut html = TERA.render(template_name, &Context::from_serialize(value)?)?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }

        if cfg!(debug_assertions) {
            fs::write(target, html)?;
        } else {
            let html = minify_html_onepass::in_place_str(&mut html, &Cfg::new())
                .map_err(|e| anyhow!("{:?}", e))?;
            fs::write(target, html)?;
        }

        Ok(())
    }
}

