use std::ops::Deref;
use std::path::Path;

use anyhow::{anyhow, Error};
use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use tl::ParserOptions;

use crate::{CONFIG, render};
use crate::config::site::Site;
use crate::pages::category::Category;
use crate::pages::page::Page;
use crate::template::model::{GLOBAL, GlobalTpl};

#[derive(Serialize)]
pub struct ArticleTpl<'a> {
    pub site: &'a GlobalTpl,
    pub category_name: &'a String,
    pub category_href: String,
    pub title: &'a String,
    pub author: &'a String,
    pub lang:&'a  String,
    pub keywords: Option<&'a String>,
    pub description: Option<&'a String>,
    pub summary: Option<&'a String>,
    pub created_at: (i32, String, String),
    pub updated_at: Option<(i32, String, String)>,
    pub toc_html: Option<&'a String>,
    pub content_html: &'a String,
}

static EMPTY: Lazy<String> = Lazy::new(|| String::new());

impl<'a> ArticleTpl<'a> {

    pub fn single(page: &'a Page) -> ArticleTpl<'a> {
        Self {
            site: GLOBAL.deref(),
            category_name: EMPTY.deref(),
            category_href: "".to_string(),
            title: &page.title,
            author: page.author.as_ref().unwrap_or(&CONFIG.site.author),
            lang: &page.lang.as_ref().unwrap_or(&CONFIG.site.lang),
            keywords: page.keywords.as_ref(),
            description: page.description.as_ref(),
            summary: page.summary.as_ref(),
            created_at: cast_date(&page.created_at),
            updated_at: page.updated_at.as_ref().map(|d| cast_date(d)),
            toc_html: page.toc_html.as_ref(),
            content_html: &page.content_html,
        }
    }

    pub fn from(page: &'a Page, c: &'a Category) -> ArticleTpl<'a> {
        ArticleTpl {
            site: GLOBAL.deref(),
            category_name: &c.name,
            category_href: render::category_url_path(c.alias.as_ref()
                .unwrap_or(&c.name)),
            title: &page.title,
            author: page.author.as_ref().unwrap_or(&CONFIG.site.author),
            lang: &page.lang.as_ref().unwrap_or(&CONFIG.site.lang),
            keywords: page.keywords.as_ref(),
            description: page.description.as_ref(),
            summary: page.summary.as_ref(),
            created_at: cast_date(&page.created_at),
            updated_at: page.updated_at.as_ref().map(|d| cast_date(d)),
            toc_html: page.toc_html.as_ref(),
            content_html: &page.content_html,
        }
    }
}

pub fn cast_date(d: &NaiveDate) -> (i32, String, String) {
    (d.year(), format!("{:02}", d.month()), format!("{:02}", d.day()))
}

