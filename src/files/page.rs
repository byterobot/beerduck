use std::ffi::OsStr;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Error};
use chrono::{Date, Datelike, NaiveDate, Utc};
use log::error;
use serde_derive::Serialize;
use tl::{Parser, ParserOptions, VDom};

use crate::config::{CONFIG, Config};
use crate::files::asciidoc::AsciiDoc;
use crate::files::render;
use crate::files::render::Template;

pub struct SinglePage {
    pub path: String,
    pub html: String,
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub permalink: String, // file name with html extension
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<NaiveDate>,
    pub date_num: Option<(i32, String, String)>,
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
    pub lang: String,
    pub html: String,
}

impl Page {
    pub fn create(path: &Path) -> Result<Self, Error> {
        let input = convert_html(path, ["-a", "nofooter"])?;
        let doc = tl::parse(&input, ParserOptions::new())?;
        let date = get_date(&doc);
        let date_num = date.as_ref().map(|v|
            (v.year(), format!("{:02}", v.month()), format!("{:02}", v.day()))
        );
        let page = Self {
            permalink: get_path(path).ok_or_else(|| anyhow!("no permalink"))?,
            title: get_title(&doc),
            author: get_author(&doc),
            date,
            date_num,
            keywords: get_keywords(&doc),
            description: None,
            lang: get_lang(&doc).unwrap_or_else(|| CONFIG.site.lang.clone()),
            html: get_body(&doc).unwrap()
        };

        Ok(page)
    }

    pub fn render(&mut self) -> Result<(), Error> {
        let v = PageRender { page: &self, config: CONFIG.deref() };
        self.html = Template::Page.render(&v)?;
        Ok(())
    }
}

fn get_date(doc: &VDom) -> Option<NaiveDate> {
    let v = doc.get_element_by_id("revdate")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    let date = NaiveDate::parse_from_str(v.as_ref(), "%Y-%m-%d")
        .expect("error date format, must `y-m-d`");
    Some(date)
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

fn get_title(doc: &VDom) -> Option<String> {
    let title = doc.query_selector("title")?
        .next()?
        .get(doc.parser())?
        .inner_text(doc.parser()).to_string();
    Some(title)
}

fn get_path(path: &Path) -> Option<String> {
    Some(format!("{}.html", path.file_stem()?.to_str()?))
}

fn convert_html<S, I>(file: &Path, args: I) -> Result<String, Error>
    where I: IntoIterator<Item = S>,
          S: AsRef<OsStr> {

    let temp_dir = CONFIG.temp_dir();
    let doc = AsciiDoc::from(&fs::read_to_string(file)?);
    let input = temp_dir.join(file.file_name().unwrap());
    fs::write(&input, doc.text())?;

    let name = input.file_name().ok_or(anyhow!("no file name for {:?}", file))?;
    let output = temp_dir.join(name).with_extension("html");

    let status = Command::new("asciidoctor")
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

#[derive(Serialize)]
pub struct PageRender<'a> {
    pub page: &'a Page,
    pub config: &'a Config,
}

#[cfg(test)]
mod test {
    use std::env::current_dir;
    use std::ops::Deref;

    use futures_await_test::async_test;
    use tera::{Context, Tera};

    use crate::config::CONFIG;
    use crate::files::page::{Page, PageRender};
    use crate::files::render::Template;

    #[async_test]
    async fn test() {
        let input = current_dir().unwrap().join("test.adoc");
        let page = Page::create(&input).unwrap();
        // println!("{:?}", page);

        let v = PageRender { page: &page, config: CONFIG.deref() };
        let r = Template::Page.render(&v).unwrap();

        // let c = Context::from_serialize(&v).unwrap();
        // let mut tera = Tera::new("templates/*.html").unwrap();
        // tera.autoescape_on(Vec::new());
        // let r = tera.render("page.html", &c).unwrap();
        println!("{}", r);

    }

}