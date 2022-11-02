use std::path::Path;

use anyhow::Error;
use chrono::NaiveDate;
use tl::ParserOptions;

use asciidoc::{self, *};

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
    pub fn from(file: &Path) -> Self {
        Self::parse(file).unwrap()
    }

    fn parse(file: &Path) -> Result<Self, Error> {
        let html = asciidoc::convert(file)?;
        let mut doc = tl::parse(&html, ParserOptions::new())?;

        // 提取dom
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
            images: resolve_img(&mut doc).unwrap_or_default(),
        };
        Ok(page)
    }
}