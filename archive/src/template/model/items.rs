use serde_derive::Serialize;

use crate::template::model::GlobalTpl;

#[derive(Serialize)]
pub struct ItemsTpl<'a> {
    pub site: &'a GlobalTpl,
    pub title: String,
    pub items: Vec<Item<'a>>,
}

#[derive(Default, Serialize)]
pub struct Item<'a> {
    pub title: String,
    pub href: String,
    pub category: String,
    pub category_href: String,
    pub pin: bool,
    pub created_at: (i32, String, String),
    pub summary: Option<&'a String>,
}
