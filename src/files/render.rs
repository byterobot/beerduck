use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::{anyhow, Error};
use once_cell::sync::Lazy;
use serde::Serialize;
use tera::{Context, Tera};

use crate::config::CONFIG;
use crate::files::asciidoc::AsciiDoc;

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
            Template::Page => "article.html",
            Template::Category => "category.html",
            Template::Categories => "categories.html",
            Template::Index => "index.html",
        };
        Ok(TERA.render(template_name, &Context::from_serialize(value)?)?)
    }
}

pub fn convert_adoc<S, I>(file: &Path, args: I) -> Result<String, Error>
    where I: IntoIterator<Item = S>,
          S: AsRef<OsStr> {

    let temp_dir = CONFIG.temp_dir();
    let doc = AsciiDoc::from(&fs::read_to_string(file)?);
    let input = temp_dir.join(file.file_name().unwrap());
    fs::write(&input, doc.text())?;

    let name = input.file_name().ok_or(anyhow!("no file name for {:?}", file))?;
    let output = temp_dir.join(name).with_extension("html");

    let status = Command::new("asciidoctor")
        .args(["-r", "asciidoctor-html5s", "-b", "html5s"])
        // .arg("-a")
        // .arg("nofooter")
        .args(args)
        .arg(input)
        .arg("-o")
        .arg(output.as_path())
        .status()?;

    match status.success() {
        true => Ok(fs::read_to_string(output.as_path())?),
        _ => Err(anyhow!("render file exit error")),
    }
}