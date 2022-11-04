use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;
use serde_derive::Serialize;

use config::site;

use crate::page::Category;

pub(crate) mod page;
pub(crate) mod about;
pub(crate) mod category;

pub fn site_tpl<'a>() -> &'a SiteTpl<'a> {
    &SITE_TPL
}

#[derive(Serialize)]
pub struct SiteTpl<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub subtitle: &'a str,
    pub copyright: &'a str,
    pub footnote: &'a str,
    pub lang: &'a str,
    pub categories_href: &'a str,
    pub about_href: &'a str,
}

impl<'a> SiteTpl<'a> {
    pub fn new() -> Self {
        Self {
            title: &site().title,
            author: &site().author,
            subtitle: &site().subtitle,
            copyright: &site().copyright,
            footnote: &site().footnote,
            lang: &site().lang,
            categories_href: "/categories.html",
            about_href: "/about.html",
        }
    }
}

static SITE_TPL: Lazy<SiteTpl> = Lazy::new(|| SiteTpl::new());

pub fn page_url<'a>(file_stem: &str, date: &'a NaiveDate, category: Option<&Category>) -> String {
    match category {
        Some(category) => {
            let url_path = url_path(date, category);
            match url_path.is_empty() {
                true => format!("/{}.html", file_stem),
                _ => format!("/{}/{}.html", url_path, file_stem),
            }
        },
        _ => format!("/{}.html", file_stem),
    }
}

fn url_path(date: &NaiveDate, category: &Category) -> String {
    if category.topic {
        return category.name.to_string();
    }
    let mut text = String::new();
    for v in site().slug.split("/").map(|v| v.trim()).filter(|v| !v.is_empty()) {
        if !text.is_empty() { text.push('/'); }
        match v {
            "y" | "Y" => text.push_str(&format!("{:02}", date.month())),
            "m" | "M" => text.push_str(&date.year().to_string()),
            "d" | "D" => text.push_str(&format!("{:02}", date.day())),
            _ => text.push_str(v),
        }
    }
    text
}
