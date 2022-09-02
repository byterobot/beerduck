use anyhow::Error;
use once_cell::sync::Lazy;
use serde::Serialize;
use tera::{Context, Tera};

use crate::config::CONFIG;

static TERA: Lazy<Tera> = Lazy::new(|| {
    let dir = CONFIG.dir.templates.join("*.html");
    let dir = dir.to_str().expect("Invalid directory for templates");
    let mut tera = Tera::new(dir).expect("new tera error");
    tera.autoescape_on(Vec::new());
    tera
});

pub enum Template {
    Page, Category, Categories, Index,
}

impl Template {
    pub fn render(&self, value: impl Serialize) -> Result<String, Error> {
        let template_name = match self {
            Template::Page => "page.html",
            Template::Category => "category.html",
            Template::Categories => "categories.html",
            Template::Index => "index.html",
        };
        Ok(TERA.render(template_name, &Context::from_serialize(value)?)?)
    }
}