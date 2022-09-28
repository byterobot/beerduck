use once_cell::sync::Lazy;
use serde_derive::Serialize;

use crate::config::CONFIG;
use crate::config::site::Site;

pub(crate) mod article;
pub(crate) mod about;
pub(crate) mod items;

pub static GLOBAL: Lazy<GlobalTpl> = Lazy::new(|| GlobalTpl::from(&CONFIG.site) );

#[derive(Serialize)]
pub struct GlobalTpl {
    pub title: String,
    pub author: String,
    pub subtitle: Option<String>,
    pub copyright: Option<String>,
    pub footnote: Option<String>,
    pub toc_title: String,
    pub categories_href: String,
    pub about_href: String,
}

impl GlobalTpl {
    fn from(site: &Site) -> Self {
        Self {
            title: site.title.clone(),
            author: site.author.clone(),
            subtitle: site.subtitle.clone(),
            copyright: site.copyright.clone(),
            footnote: site.footnote.clone(),
            toc_title: site.toc_title.clone(),
            categories_href: "/categories.html".into(),
            about_href: "/about.html".into(),
        }
    }
}
