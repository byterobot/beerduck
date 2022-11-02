use std::borrow::Cow;

use chrono::NaiveDate;
use log::error;
use once_cell::sync::Lazy;
use regex::Regex;
use tl::{NodeHandle, VDom};

pub fn resolve_img(dom: &mut VDom) -> Option<Vec<String>> {
    let mut list = vec![];
    for n in dom.query_selector("img[src]")?.collect::<Vec<NodeHandle>>() {
        if let Some(v) = n.get_mut(dom.parser_mut()) {
            if let Some(v) = v.as_tag_mut() {
                if let Some(Some(v)) = v.attributes_mut().get_mut("src") {
                    match String::from_utf8(v.as_bytes().to_vec()) {
                        Ok(src) => {
                            let _ = v.set(&*resolve_image_path(&src));
                            list.push(src);
                        }
                        Err(e) => {
                            error!("parse img src error: {}", e);
                        }
                    }
                }
            }
        }
    }

    Some(list)
}

fn resolve_image_path(path: &str) -> String {
    format!("/static/images/{}", make_relative_path(path))
}

fn make_relative_path(txt: &str) -> Cow<str> {
    REG_ABSOLUTE.replace(txt, "")
}

// static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(adoc)$").unwrap());
static REG_ABSOLUTE: Lazy<Regex> = Lazy::new(|| Regex::new("^/").unwrap());

// ----------------- Extract dom info --------------------//

pub fn get_title(doc: &VDom) -> Option<String> {
    let title = doc.query_selector("title")?
        .next()?
        .get(doc.parser())?
        .inner_text(doc.parser()).to_string();
    Some(title)
}

pub fn get_author(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("author")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    Some(v.as_ref().to_string())
}

pub fn get_lang(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"html[lang]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("lang")??;
    let b = String::from_utf8(a.as_bytes().to_vec()).ok()?;
    Some(b)
}

pub fn get_keywords(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"meta[name="keywords"]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("content")??;
    String::from_utf8(a.as_bytes().to_vec()).ok()
}

pub fn get_description(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"meta[name="description"]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("content")??;
    String::from_utf8(a.as_bytes().to_vec()).ok()
}

pub fn get_toc(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("toc")?
        .get(doc.parser())?
        .children()?
        .all(doc.parser())
        .get(2)?
        .outer_html(doc.parser());
    Some(v.to_string())
}

pub fn get_date(doc: &VDom) -> Option<NaiveDate> {
    let v = doc.get_element_by_id("revdate")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    let date = NaiveDate::parse_from_str(v.as_ref(), "%Y-%m-%d")
        .expect("error date format, must `yyyy-mm-dd` format");
    Some(date)
}

pub fn get_content(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("content")?
        .get(doc.parser())?
        .outer_html(doc.parser());
    Some(v.trim().to_string())
}