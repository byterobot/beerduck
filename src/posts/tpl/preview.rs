use chrono::Datelike;
use serde_derive::Serialize;

use crate::config::CONFIG;
use crate::config::site::Site;
use crate::posts::{Category, Generated, Preview};

#[derive(Serialize)]
pub struct IndexTpl<'a> {
    site: &'a Site,
    items: Vec<ArticleItem>,
}

impl<'a> IndexTpl<'a> {
    pub fn from(g: Generated) -> Self {
        Self { site: &CONFIG.site, items: build_items(g.items.as_slice()), }
    }
}

#[derive(Serialize)]
pub struct CategoryTpl<'a> {
    site: &'a Site,
    title: String,
    items: Vec<ArticleItem>,
}

impl<'a> CategoryTpl<'a> {
    pub fn from(g: &Generated) -> Self {
        Self { site: &CONFIG.site, title: g.title.clone(), items: build_items(g.items.as_slice()), }
    }
}

fn build_items(items: &[Preview]) -> Vec<ArticleItem> {
    items.iter().map(|v| {
        let c = &v.created_at;
        ArticleItem {
            title: v.title.clone(),
            href: v.href.clone(),
            pin: v.pin,
            created_at: (c.year(), format!("{:02}", c.month()), format!("{:02}", c.day())),
            summary: v.summary.clone(),
            category: v.category.clone(),
            category_href: v.category_href.clone()
        }
    }).collect()
}

#[derive(Serialize)]
pub struct ArticleItem {
    title: String,
    href: String,
    pin: bool,
    created_at: (i32, String, String),
    summary: Option<String>,
    category: String,
    category_href: String,
}


#[derive(Serialize)]
pub struct CategoriesTpl<'a> {
    site: &'a Site,
    items: Vec<CategoryItem>,

}

impl<'a> CategoriesTpl<'a> {
    pub fn from(g: &Generated) -> Self {
        let items = g.items.iter().map(|v|
            CategoryItem {
                category: v.category.clone(),
                category_href: v.category_href.clone(),
            }
        ).collect();

        Self { site: &CONFIG.site, items }
    }
}

#[derive(Serialize)]
pub struct CategoryItem {
    category: String,
    category_href: String,
}