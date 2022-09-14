use std::ops::Deref;
use std::path::Path;

use anyhow::{anyhow, Error};
use chrono::{Datelike, NaiveDate};
use serde_derive::{Deserialize, Serialize};
use tl::ParserOptions;

use crate::config::site::Site;
use crate::convert;
use crate::posts::{Category, TextFile};
use crate::posts::page::Page;
use crate::posts::tpl::{GLOBAL, Global};

#[derive(Serialize)]
pub struct ArticleTpl<'a> {
    pub site: &'a Global,
    pub category_name: String,
    pub category_href: String,
    pub title: String,
    pub author: String,
    pub lang: String,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub created_at: (i32, String, String),
    pub updated_at: Option<(i32, String, String)>,
    pub nav_html: Option<String>,
    pub content_html: String,
}

pub fn build_tpl(page: Page, category: &Category) -> Result<ArticleTpl, Error> {
    let c = page.created_at;
    let category_link = category.href();
    let created_at = (c.year(), format!("{:02}", c.month()), format!("{:02}", c.day()));
    let updated_at = page.updated_at.as_ref().map(|u|
        (u.year(), format!("{:02}", u.month()), format!("{:02}", u.day()))
    );

    let tpl = ArticleTpl {
        site: GLOBAL.deref(),
        category_name: category.name.clone(),
        category_href: category.href(),
        title: page.title,
        author: page.author,
        lang: page.lang,
        keywords: page.keywords,
        description: page.description,
        summary: page.summary,
        created_at,
        updated_at,
        nav_html: page.nav_html,
        content_html: page.content_html,
    };
    Ok(tpl)
}
