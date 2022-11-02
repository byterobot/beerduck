use std::ops::Deref;
use std::path::Path;

use anyhow::{anyhow, Error};
use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;
use serde_derive::Serialize;

use crate::page::{Article, Category};
use crate::template::{site_tpl, SiteTpl};

#[derive(Serialize)]
pub struct PageTpl<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub lang:&'a str,
    pub keywords: Option<&'a str>,
    pub description: Option<&'a str>,
    pub summary: Option<&'a str>,
    pub created_at: (i32, String, String),
    pub updated_at: Option<(i32, String, String)>,
    pub toc_html: Option<&'a str>,
    pub content_html: &'a str,

    pub single_page: bool,

    pub category_name: &'a str,
    pub category_href: String,

    pub site: &'a SiteTpl<'a>,
}

impl<'a> PageTpl<'a> {

    pub fn single(a: &'a Article) -> PageTpl<'a> {
        let site = site_tpl();
        Self {
            title: a.title.as_ref().map(|v| v.as_str()).unwrap_or("Untitled"),
            author: a.author.as_ref().map(|v| v.as_str()).unwrap_or(site.author),
            lang: a.lang.as_ref().map(|v| v.as_str()).unwrap_or(site.lang),
            keywords: a.keywords.as_ref().map(|v| v.as_str()),
            description: a.description.as_ref().map(|v| v.as_str()),
            summary: a.summary.as_ref().map(|v| v.as_str()),
            created_at: cast_date(&a.created_at.unwrap_or_default()),
            updated_at: a.updated_at.as_ref().map(|d| cast_date(d)),
            toc_html: a.toc_html.as_ref().map(|v| v.as_str()),
            content_html: a.content_html.as_ref().map(|v| v.as_str()).unwrap_or(""),

            single_page: true,
            category_name: "",
            category_href: "".to_string(),
            site: site_tpl(),
        }
    }

    pub fn from(a: &'a Article, c: &'a Category) -> PageTpl<'a> {
        let mut tpl = Self::single(a);
        tpl.single_page = false;
        tpl.category_name = &c.show_name;
        tpl.category_href = format!("/categories/{}.html", c.name);
        tpl
    }
}

pub fn cast_date(d: &NaiveDate) -> (i32, String, String) {
    (d.year(), format!("{:02}", d.month()), format!("{:02}", d.day()))
}

