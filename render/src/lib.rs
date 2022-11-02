use std::fs;
use std::path::Path;

use anyhow::{anyhow, Error};
use bimap::BiMap;
use minify_html_onepass::Cfg;
use once_cell::sync::Lazy;
use serde::Serialize;
use tera::{Context, Tera};

use config::{dev_mode, workspace};

use crate::register::register;

mod register;

#[derive(Eq, PartialEq, Hash)]
pub enum Template {
    Article, Category, Categories, Index, About,
}

impl Template {
    pub fn render_write(&self, value: impl Serialize, target: &Path) -> Result<(), Error> {
        let template_name = TEMPLATES.get_by_left(self).unwrap();
        let mut html = TERA.render(template_name, &Context::from_serialize(value)?)?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }

        let html = match dev_mode() {
            true => html.as_str(),
            _ => minify_html_onepass::in_place_str(&mut html, &Cfg::new())
                .map_err(|e| anyhow!("{:?}", e))?
        };

        Ok(fs::write(target, html)?)
    }
}

static TERA: Lazy<Tera> = Lazy::new(|| {
    let dir = workspace().theme.templates.join("*.html");
    let mut tera = Tera::new(dir.to_str().unwrap()).unwrap();
    tera.full_reload().unwrap();
    tera.autoescape_on(Vec::new());
    register(&mut tera);
    tera
});

static TEMPLATES: Lazy<BiMap<Template, &'static str>> = Lazy::new(||
    BiMap::from_iter([
        (Template::Article, "article.html"),
        (Template::Category, "category.html"),
        (Template::Categories, "categories.html"),
        (Template::Index, "index.html"),
        (Template::About, "about.html"),
    ])
);
