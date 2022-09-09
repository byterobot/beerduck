use chrono::{Datelike, NaiveDate};
use serde_derive::Serialize;

use crate::config::CONFIG;
use crate::config::site::Site;
use crate::dict::DICT;
use crate::site::article::Page;

#[derive(Serialize)]
pub struct PageTpl<'a> {
    pub site: &'a Site,
    pub category_name: &'a String,
    pub category_link: String,
    // pub category_nav: bool,
    pub title: &'a String,
    pub author: &'a String,
    pub lang: &'a String,
    pub keywords: Option<&'a String>,
    pub description: Option<&'a String>,
    pub summary: Option<&'a String>,
    pub created_at: &'a NaiveDate,
    pub created_at_num: (i32, String, String),
    pub updated_at: Option<&'a NaiveDate>,
    pub nav_html: Option<&'a String>,
    pub content_html: &'a String,
}

impl<'a> PageTpl<'a> {
    pub fn from(page: &'a Page, nav_html: Option<&'a String>, content_html: &'a String) -> Self {
        let c = &page.created_at;
        let category_name = DICT.get_category_name(&page.file_name)
            .expect("the category name not found.");
        let category = DICT.get_category(category_name).expect("the category not found");
        let category_link = category.html_relative.to_str().unwrap().to_string();

        PageTpl {
            site: &CONFIG.site,
            category_name,
            category_link: category.html_relative.to_str().unwrap().to_string(),
            // category_nav: false,
            title: &page.title,
            author: &page.author,
            lang: &page.lang,
            keywords: page.keywords.as_ref(),
            description: page.description.as_ref(),
            summary: page.summary.as_ref(),
            created_at: &page.created_at,
            created_at_num: (c.year(), format!("{:02}", c.month()), format!("{:02}", c.day())),
            updated_at: page.updated_at.as_ref(),
            nav_html,
            content_html,
        }
    }
}