use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Error};
use chrono::{Date, NaiveDate, Utc};
use log::error;
use tl::{Parser, ParserOptions, VDom};

use crate::config::CONFIG;
use crate::files::asciidoc::AsciiDoc;

pub struct SinglePage {
    pub path: String,
    pub html: String,
}

#[derive(Debug)]
pub struct Page {
    pub path: String,
    pub title: String,
    pub author: Option<String>,
    pub date: Date<Utc>, // if not exists, assign current date.
    pub lang: String,
    pub keywords: Vec<String>,
    pub html: String,
}

impl Page {
    pub fn create(path: &Path) -> Result<Self, Error> {
        let input = render_html(path)?;
        let doc = tl::parse(&input, ParserOptions::new())?;

        let page = Self {
            path: get_path(path)?,
            title: get_title(&doc)?,
            author: get_author(&doc),
            date: get_date(&doc).unwrap_or_else(|| Utc::today()),
            keywords: get_keywords(&doc).unwrap_or_else(|| Vec::new()),
            lang: get_lang(&doc).unwrap_or_else(|| "en".to_string()),
            html: get_body(&doc).unwrap()
        };

        Ok(page)
    }
}

fn get_date(doc: &VDom) -> Option<Date<Utc>> {
    let v = doc.get_element_by_id("revdate")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    match NaiveDate::parse_from_str(v.as_ref(), "%Y-%m-%d") {
        Ok(v) => Some(Date::from_utc(v, Utc)),
        Err(e) => {
            error!("error date format");
            None
        }
    }
}

fn get_body(doc: &VDom) -> Option<String> {
    let v = doc.query_selector("body")?.next()?
        .get(doc.parser())?
        .inner_html(doc.parser());
    Some(v.trim().to_string())
}

fn get_keywords(doc: &VDom) -> Option<Vec<String>> {
    let a = doc.query_selector(r#"meta[name="keywords"]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("content")??;
    let b = String::from_utf8(a.as_bytes().to_vec()).ok()?;
    Some(b.split(',').map(|v| v.to_string()).collect())
}

fn get_lang(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"html[lang]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("lang")??;
    let b = String::from_utf8(a.as_bytes().to_vec()).ok()?;
    Some(b)
}

fn get_author(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("author")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    Some(v.as_ref().to_string())
}

fn get_title(doc: &VDom) -> Result<String, Error> {
    let title = doc.query_selector("title")
        .ok_or_else(|| anyhow!("No title tag"))?
        .next()
        .ok_or_else(|| anyhow!("No title tag"))?
        .get(doc.parser())
        .ok_or_else(|| anyhow!("No title tag"))?
        .inner_text(doc.parser()).to_string();
    Ok(title)
}

fn get_path(path: &Path) -> Result<String, Error> {
    let name = path.file_stem().ok_or(anyhow!("no file name: {:?}", path))?;
    let name = name.to_str().ok_or(anyhow!("no file name: {:?}", path))?;
    Ok(format!("{}.html", name))
}

fn render_html(path: &Path) -> Result<String, Error> {
    let temp_dir = CONFIG.temp_dir();
    let doc = AsciiDoc::from(&fs::read_to_string(path)?);
    let input = temp_dir.join(path.file_name().unwrap());
    fs::write(&input, doc.text())?;

    let name = input.file_name().ok_or(anyhow!("no file name for {:?}", path))?;
    let output = temp_dir.join(name).with_extension("html");

    let status = Command::new("asciidoctor")
        .arg("-a")
        .arg("nofooter")
        .arg(input)
        .arg("-o")
        .arg(output.as_path())
        .status()?;

    match status.success() {
        true => Ok(fs::read_to_string(output.as_path())?),
        _ => Err(anyhow!("render file exit error")),
    }
}

#[cfg(test)]
mod test {
    use std::env::current_dir;

    use futures_await_test::async_test;

    use crate::files::page::Page;

    #[async_test]
    async fn test() {
        let a = current_dir().unwrap();
        // println!("current: {:?}", a);

        let input = a.as_path().join("test.adoc");
        // println!("input: {:?}", input.as_path());

        let page = Page::create(&input).unwrap();
        println!("{:?}", page);
    }
}