use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Error};
use once_cell::sync::Lazy;
use serde::Serialize;
use tera::{Context, Tera};

use crate::config::CONFIG;
use crate::asciidoc::AsciiDoc;

static TERA: Lazy<Tera> = Lazy::new(|| {
    let dir = CONFIG.workspace.templates.join("*.html");
    let dir = dir.to_str().expect("Invalid directory for templates");
    let mut tera = Tera::new(dir).expect("new tera error");
    tera.autoescape_on(Vec::new());
    tera
});

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

        let html = TERA.render(template_name, &Context::from_serialize(value)?)?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, html)?;

        Ok(())
    }
}

/// Covert adoc file to html.
pub fn convert_adoc(adoc: &Path) -> Result<String, Error> {
    let temp_dir = CONFIG.workspace.temp.as_path();
    fs::create_dir_all(temp_dir)?;
    let doc = AsciiDoc::from(&fs::read_to_string(adoc)?);
    let input = temp_dir.join(adoc.file_name().unwrap());
    fs::write(&input, doc.text())?;

    let name = input.file_name().ok_or(anyhow!("no file name for {:?}", adoc))?;
    let output = temp_dir.join(name).with_extension("html");

    let status = Command::new("asciidoctor")
        .args(["-r", "asciidoctor-html5s", "-b", "html5s"])
        // .arg("-a")
        // .arg("nofooter")
        .arg(input)
        .arg("-o")
        .arg(output.as_path())
        .status()?;

    match status.success() {
        true => Ok(fs::read_to_string(output.as_path())?),
        _ => Err(anyhow!("render file exit error")),
    }
}
