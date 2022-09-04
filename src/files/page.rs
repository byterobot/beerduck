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
use crate::files::template::PageTpl;

// pub struct SimplePage {
//     pub permalink: String,
//     pub html: String,
// }

#[derive(Debug, Default, Serialize)]
pub struct Page {
    pub autogen: bool,
    pub permalink: String, // file name with html extension
    pub title: String,
    pub author: String,
    pub lang: String,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub created_at: NaiveDate,
    pub created_at_num: (i32, String, String),
    pub updated_at: Option<NaiveDate>,

    pub nav_html: Option<String>, // id "toc"
    pub content_html: String, // id "content"
    pub full_html: String,
}

pub fn render_template(tpl: Template, pages: &[Page]) -> Result<Page, Error> {
    todo!()
}

// render asciidoc
pub fn render(file: &Path) -> Result<Page, Error> {
    let temp_html = render::convert_adoc(file, ["-a", "nofooter"])?;
    let doc = tl::parse(&temp_html, ParserOptions::new())?;
    let date = get_date(&doc);
    let date_num = date.as_ref().map(|v|
        (v.year(), format!("{:02}", v.month()), format!("{:02}", v.day()))
    );
    let mut page = Page {
        autogen: false,
        permalink: get_path(file).ok_or_else(|| anyhow!("missing permalink"))?,
        title: get_title(&doc).ok_or_else(|| anyhow!("missing title"))?,
        author: get_author(&doc).ok_or_else(|| anyhow!("missing author"))?,
        lang: get_lang(&doc).unwrap_or_else(|| CONFIG.site.lang.clone()),
        keywords: get_keywords(&doc),
        description: None,
        summary: None,
        created_at: date.ok_or_else(|| anyhow!("missing created date"))?,
        created_at_num: date_num.ok_or_else(|| anyhow!("missing created date"))?,
        updated_at: None,
        nav_html: get_nav(&doc),
        content_html: get_content(&doc).ok_or_else(|| anyhow!("missing content"))?,
        full_html: Default::default(),
    };
    page.full_html = Template::Page.render(&PageTpl::from(&page))?;

    Ok(page)
}

fn get_content(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("content")?
        .get(doc.parser())?
        .outer_html(doc.parser());
    Some(v.trim().to_string())
}

fn get_nav(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("toc")?
        .get(doc.parser())?
        .outer_html(doc.parser());
    Some(v.trim().to_string())
}

fn get_date(doc: &VDom) -> Option<NaiveDate> {
    let v = doc.get_element_by_id("revdate")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    let date = NaiveDate::parse_from_str(v.as_ref(), "%Y-%m-%d")
        .expect("error date format, must `yyyy-mm-dd` format");
    Some(date)
}

fn get_keywords(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"meta[name="keywords"]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("content")??;
    String::from_utf8(a.as_bytes().to_vec()).ok()
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


#[cfg(test)]
mod test {
    use std::env::current_dir;
    use std::ops::Deref;

    use futures_await_test::async_test;
    use tera::{Context, Tera};

    use crate::config::CONFIG;
    use crate::files::page::{Page, render};
    use crate::files::render::Template;

    #[async_test]
    async fn test() {
        let input = current_dir().unwrap().join("test.adoc");
        let page = render(&input).unwrap();
        println!("{}", page.full_html);

        // let v = PageRender { page: &page, config: CONFIG.deref() };
        // let r = Template::Page.render(&v).unwrap();

        // let c = Context::from_serialize(&v).unwrap();
        // let mut tera = Tera::new("templates/*.html").unwrap();
        // tera.autoescape_on(Vec::new());
        // let r = tera.render("page.html", &c).unwrap();
        // println!("{}", r);

    }

}