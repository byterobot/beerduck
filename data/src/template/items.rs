use chrono::NaiveDate;
use serde_derive::Serialize;

use crate::generate::page::page_url;
use crate::page::{Article, Category};
use crate::template::{site_tpl, SiteTpl};
use crate::template::page::cast_date;

#[derive(Serialize)]
pub struct ItemsTpl<'a> {
    pub title: &'a str,
    pub items: Vec<Item<'a>>,
    pub site: &'a SiteTpl<'a>,
}

#[derive(Default, Serialize)]
pub struct Item<'a> {
    pub title: &'a str,
    pub href: String,
    pub category: &'a str,
    pub category_href: String,
    // pub pin: bool,
    pub created_at: (i32, String, String),
    pub summary: Option<&'a str>,
}

impl<'a> ItemsTpl<'a> {
    pub fn from(articles: &'a [(String, Article)], category: &'a Category) -> Self {
        let mut items = vec![];
        for (file_stem, article) in articles {
            let date = article.created_at.as_ref().unwrap_or(&NaiveDate::MIN);
            let item = Item {
                title: article.title.as_ref().map(|v| v.as_str()).unwrap_or("Untitled"),
                href: page_url(file_stem, date, Some(category)),
                category: &category.show_name,
                category_href: format!("/categories/{}.html", category.name),
                created_at: cast_date(date),
                summary: article.summary.as_ref().map(|v| v.as_str())
            };
            items.push(item);
        }
        Self { title: &category.show_name, items, site: site_tpl(), }
    }
}
