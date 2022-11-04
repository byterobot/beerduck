use once_cell::sync::Lazy;
use serde_derive::Serialize;

use config::site;

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