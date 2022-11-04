use chrono::NaiveDate;
use serde_derive::Serialize;

use crate::page::{Article, Category};
use crate::template::{page_url, site_tpl, SiteTpl};
use crate::template::page::cast_date;

#[derive(Serialize)]
pub struct CategoryTpl<'a> {
    pub title: &'a str,
    pub items: Vec<ArticleItem<'a>>,
    pub site: &'a SiteTpl<'a>,
}

#[derive(Default, Serialize)]
pub struct ArticleItem<'a> {
    pub title: &'a str,
    pub href: String,
    pub category: &'a str,
    pub category_href: String,
    // pub pin: bool,
    pub created_at: (i32, String, String),
    pub summary: Option<&'a str>,
}

impl<'a> CategoryTpl<'a> {
    pub fn from(articles: &'a [(String, Article)], category: &'a Category) -> Self {
        let mut items = vec![];
        for (file_stem, article) in articles {
            let date = article.created_at.as_ref().unwrap_or(&NaiveDate::MIN);
            let item = ArticleItem {
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


#[derive(Serialize)]
pub struct CategoriesTpl<'a> {
    pub items: Vec<CategoryItem<'a>>,
    pub site: &'a SiteTpl<'a>,
}

#[derive(Default, Serialize)]
pub struct CategoryItem<'a> {
    pub title: &'a str,
    pub href: String,
    // pub category: &'a str,
    // pub category_href: String,
    // pub pin: bool,
}

impl<'a> CategoriesTpl<'a> {
    pub fn from(categories: &'a [Category]) -> Self {
        let items = categories.into_iter().map(|c| CategoryItem {
            title: &c.show_name,
            href: format!("/categories/{}.html", c.name),
        }).collect();
        Self { items, site: site_tpl(), }
    }
}


#[derive(Serialize)]
pub struct IndexTpl<'a> {
    pub items: Vec<ArticleItem<'a>>,
    pub site: &'a SiteTpl<'a>,
}

impl<'a> IndexTpl<'a> {
    pub fn from(items: Vec<ArticleItem<'a>>) -> Self {
        Self { items, site: site_tpl() }
    }
}
