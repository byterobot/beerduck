use std::path::Path;

use anyhow::{anyhow, Error};
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use tl::ParserOptions;

use crate::config::CONFIG;
use crate::render;

mod dom;
mod template;

#[derive(Debug, Default, Serialize)]
pub struct Page {
    pub autogen: bool, // 是的话, header 只显示标题,其他都不显示
    // pub category_name: String,
    // pub category_permalink: String,
    pub file_name: String, // file_name.adoc
    pub title: String,
    pub author: String,
    pub lang: String,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub created_at: NaiveDate,
    // pub created_at_num: (i32, String, String),
    pub updated_at: Option<NaiveDate>,

    // pub nav_html: Option<String>, // id "toc"
    // pub content_html: String, // id "content"
    // pub full_html: String,
}

pub fn render(adoc: &Path, target: &Path) -> Result<Page, Error> {
    // 转换成 html
    let html = render::convert(adoc)?;
    let doc = tl::parse(&html, ParserOptions::new())?;
    // 提取dom
    let page = Page {
        autogen: false,
        file_name: adoc.file_name().unwrap().to_str().unwrap().to_string(), // 理应存在
        title: dom::get_title(&doc).ok_or_else(|| anyhow!("missing title"))?,
        author: dom::get_author(&doc).ok_or_else(|| anyhow!("missing author"))?,
        lang: dom::get_lang(&doc).unwrap_or_else(|| CONFIG.site.lang.clone()),
        keywords: dom::get_keywords(&doc),
        description: dom::get_description(&doc),
        summary: None,
        created_at: dom::get_date(&doc).ok_or_else(|| anyhow!("missing created date"))?,
        updated_at: None
    };

    // todo 从全局信息中获取上一级category

    let nav_html = dom::get_nav(&doc);
    let content_html = dom::get_content(&doc).ok_or_else(|| anyhow!("missing content"))?;

    // 渲染
    // 写入目的地

    Ok(page)
}
