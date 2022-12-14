use std::fs;
use std::path::Path;

use anyhow::Error;
use chrono::NaiveDate;
use tl::ParserOptions;

use asciidoc::*;

// asciidoc file contents
pub struct Article {
    pub title: Option<String>,
    pub author: Option<String>,
    pub lang: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub created_at: Option<NaiveDate>,
    pub updated_at: Option<NaiveDate>,
    pub toc_html: Option<String>, // id "toc>ol"
    pub content_html: Option<String>, // id "content"
    pub images: Vec<String>,
}

impl Article {
    pub fn from(file: &Path) -> Result<Self, Error> {
        let txt = fs::read_to_string(file)?;
        let html = convert(&txt)?;
        let mut doc = tl::parse(&html, ParserOptions::new())?;
        let images = get_content_images(&doc).unwrap_or_default();
        resolve_images(&mut doc);

        let page = Article {
            title: get_title(&doc),
            author: get_author(&doc),
            lang: get_lang(&doc),
            keywords: get_keywords(&doc),
            description: get_description(&doc),
            summary: None,
            created_at: get_date(&doc),
            updated_at: None,
            toc_html: get_toc(&doc),
            content_html:  get_content(&doc),
            images,
        };
        Ok(page)
    }
}