use chrono::NaiveDate;
use serde_derive::Serialize;
use crate::config::CONFIG;
use crate::config::site::Site;
use crate::files::page::Page;

#[derive(Serialize)]
pub struct PageTpl<'a> {
    pub site: &'a Site,
    pub title: &'a String,
    pub author: &'a String,
    pub lang: &'a String,
    pub keywords: Option<&'a [String]>,
    pub description: Option<&'a String>,
    pub summary: Option<&'a String>,
    pub body: &'a String, // 已组织好的
    pub created_at: &'a NaiveDate,
    pub created_at_num: &'a (i32, String, String),
    pub updated_at: Option<&'a NaiveDate>,
}

impl<'a> PageTpl<'a> {
    pub fn from(page: &'a Page) -> Self {
        PageTpl {
            site: &CONFIG.site,
            title: &page.title,
            author: &page.author,
            lang: &page.lang,
            keywords: page.keywords.as_ref().map(|v| v.as_slice()),
            description: page.description.as_ref(),
            summary: page.summary.as_ref(),
            body: &page.html,
            created_at: &page.created_at,
            created_at_num: &page.created_at_num,
            updated_at: page.updated_at.as_ref(),
        }
    }
}

// index, category, categories
#[derive(Serialize)]
pub struct CategoryTpl<'a> {
    pub site: &'a Site,
    pub title: &'a String,
    pub lang: &'a String,
    pub pages: &'a [PageTpl<'a>],
}

/*
#[derive(Serialize)]
pub struct Common<'a> {
    pub title: &'a String,
    pub subtitle: Option<&'a String>,
    pub copyright: Option<&'a String>,
    pub footnote: Option<&'a String>,
}
*/
/*
#[derive(Serialize)]
pub struct Header<'a> {
    pub title: &'a String,
    pub subtitle: Option<&'a String>,
}

#[derive(Serialize)]
pub struct Footer<'a> {
    pub copyright: Option<&'a String>,
    pub footnote: Option<&'a String>,
}*/

/*
#[derive(Serialize)]
pub struct Head<'a> {
    pub author: &'a String,
    pub lang: &'a String,
    pub keywords: Option<&'a [String]>,
    pub description: Option<&'a String>,
    pub summary: Option<&'a String>,
}*/
