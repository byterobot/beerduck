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
    pub html: String,
    pub lang: String,
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub created_at: NaiveDate,
    pub created_at_num: (i32, String, String),
    pub updated_at: Option<NaiveDate>,
}

pub fn render_template(tpl: Template, pages: &[Page]) -> Result<Page, Error> {
    todo!()
}

// render asciidoc
pub fn render(file: &Path) -> Result<Page, Error> {
    let html = convert_adoc(file, ["-a", "nofooter"])?;
    let doc = tl::parse(&html, ParserOptions::new())?;
    let date = get_date(&doc);
    let date_num = date.as_ref().map(|v|
        (v.year(), format!("{:02}", v.month()), format!("{:02}", v.day()))
    );
    let mut page = Page {
        autogen: false,
        permalink: get_path(file).ok_or_else(|| anyhow!("missing permalink"))?,
        title: get_title(&doc).ok_or_else(|| anyhow!("missing title"))?,
        author: get_author(&doc).ok_or_else(|| anyhow!("missing author"))?,
        html: get_body(&doc).ok_or_else(|| anyhow!("missing content body"))?,
        lang: get_lang(&doc).unwrap_or_else(|| CONFIG.site.lang.clone()),
        keywords: get_keywords(&doc),
        description: None,
        summary: None,
        created_at: date.ok_or_else(|| anyhow!("missing created date"))?,
        created_at_num: date_num.ok_or_else(|| anyhow!("missing created date"))?,
        updated_at: None,
    };
    page.html = Template::Page.render(&PageTpl::from(&page))?;

    Ok(page)
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

fn convert_adoc<S, I>(file: &Path, args: I) -> Result<String, Error>
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