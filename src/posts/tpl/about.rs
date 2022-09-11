use std::ops::Deref;
use std::path::Path;
use anyhow::Error;
use serde_derive::Serialize;
use crate::posts::page::Page;
use crate::posts::tpl::{Global, GLOBAL};

#[derive(Serialize)]
pub struct AboutTpl<'a> {
    pub site: &'a Global,
    pub title: String,
    pub lang: String,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub content_html: String,
}

pub fn build_tpl(adoc_file: &Path) -> Result<AboutTpl, Error> {
    let page = Page::from(adoc_file)?;
    let tpl = AboutTpl {
        site: GLOBAL.deref(),
        title: page.title,
        lang: page.lang,
        keywords: page.keywords,
        description: page.description,
        content_html: page.content_html
    };
    Ok(tpl)
}
