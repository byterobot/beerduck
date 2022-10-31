use std::path::Path;

use anyhow::Error;
use tl::ParserOptions;

use crate::asciidoc::dom::*;
use crate::page::article::Article;

pub mod hybrid;
pub mod asciidoctor;
pub mod dom;

pub fn parse_doc(adoc: &Path) -> Result<Article, Error> {
    let html = asciidoctor::convert(adoc)?;
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