use std::collections::LinkedList;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Error};
use chrono::NaiveDate;
use log::error;
use serde_derive::Serialize;
use tl::{NodeHandle, ParserOptions, VDom};
use tl::queryselector::iterable::QueryIterable;

use crate::config::CONFIG;
use crate::convert;

#[derive(Debug, Default, Serialize)]
pub struct Page {
    pub title: String,
    pub author: String,
    pub lang: String,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub created_at: NaiveDate,
    pub updated_at: Option<NaiveDate>,
    pub nav_html: Option<String>, // id "toc"
    pub content_html: String, // id "content"
    pub images: LinkedList<(String, String)>,
}

impl Page {
    pub fn from(adoc_file: &Path) -> Result<Self, Error> {
        parse(adoc_file, true)
    }

    pub fn from_simple(adoc_file: &Path) -> Result<Self, Error>{
        parse(adoc_file, false)
    }
}

fn parse(adoc: &Path, full: bool) -> Result<Page, Error> {
    // 转换成 html
    let html = convert::convert_adoc(adoc)?;
    let mut doc = tl::parse(&html, ParserOptions::new())?;

    // 提取dom
    let mut page = Page::default();
    page.title = get_title(&doc).ok_or_else(|| anyhow!("missing title"))?;
    page.author = get_author(&doc).unwrap_or_else(|| CONFIG.site.author.clone());
    page.summary = None;
    page.created_at = get_date(&doc).unwrap_or_else(|| NaiveDate::default());
    page.updated_at = None;

    if full {
        page.images = replace_img(&mut doc).unwrap_or(Default::default());
        page.lang = get_lang(&doc).unwrap_or_else(|| CONFIG.site.lang.clone());
        page.keywords = get_keywords(&doc);
        page.description = get_description(&doc);
        page.nav_html = get_nav(&doc);
        page.content_html = get_content(&doc).ok_or_else(|| anyhow!("missing content"))?;
    }

    Ok(page)
}

fn replace_img(dom: &mut VDom) -> Option<LinkedList<(String, String)>> {
    let mut list = LinkedList::new();
    for n in dom.query_selector("img[src]")?.collect::<Vec<NodeHandle>>() {
        if let Some(v) = n.get_mut(dom.parser_mut()) {
            if let Some(v) = v.as_tag_mut() {
                if let Some(Some(v)) = v.attributes_mut().get_mut("src") {
                    match String::from_utf8(v.as_bytes().to_vec()) {
                        Ok(src) => {
                            let target = format!("/static{}", src);
                            v.set(&*target);
                            list.push_back((src, target));
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

// Extract dom info

fn get_title(doc: &VDom) -> Option<String> {
    let title = doc.query_selector("title")?
        .next()?
        .get(doc.parser())?
        .inner_text(doc.parser()).to_string();
    Some(title)
}

fn get_author(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("author")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    Some(v.as_ref().to_string())
}

fn get_lang(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"html[lang]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("lang")??;
    let b = String::from_utf8(a.as_bytes().to_vec()).ok()?;
    Some(b)
}

fn get_keywords(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"meta[name="keywords"]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("content")??;
    String::from_utf8(a.as_bytes().to_vec()).ok()
}

fn get_description(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"meta[name="description"]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("content")??;
    String::from_utf8(a.as_bytes().to_vec()).ok()
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

fn get_content(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("content")?
        .get(doc.parser())?
        .outer_html(doc.parser());
    Some(v.trim().to_string())
}