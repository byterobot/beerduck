use chrono::Datelike;
use serde_derive::Serialize;
use crate::config::CONFIG;

use crate::config::site::Site;
use crate::posts::{Category, Generated};

#[derive(Serialize)]
pub struct IndexTpl {

}

#[derive(Serialize)]
pub struct CategoryTpl<'a> {
    site: &'a Site,
    title: String,
    items: Vec<Item>,
}

impl<'a> CategoryTpl<'a> {
    pub fn create(g: &Generated) -> Self {
        // let items = vec![];
        let items = g.items.iter().map(|v| {
            let c = &v.created_at;
            Item {
                title: v.title.clone(),
                pin: v.pin,
                created_at: (c.year(), format!("{:02}", c.month()), format!("{:02}", c.day())),
                summary: v.summary.clone(),
                category: v.category.clone(),
                category_href: v.category_href.clone()
            }
        }).collect();

        Self { site: &CONFIG.site, title: g.title.clone(), items, }
    }
}


#[derive(Serialize)]
pub struct CategoriesTpl<'a> {
    site: &'a Site,
    // title: String,

}

impl<'a> CategoriesTpl<'a> {
    pub fn create(g: &Generated) -> Self {
        todo!()
    }
}

#[derive(Serialize)]
pub struct Item {
    title: String,
    pin: bool,
    created_at: (i32, String, String),
    summary: Option<String>,
    category: String,
    category_href: String,
}