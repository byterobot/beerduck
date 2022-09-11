use once_cell::sync::Lazy;
use serde_derive::Serialize;

use crate::config::CONFIG;
use crate::posts::{Posts, POSTS};

pub mod about;
pub mod article;
pub mod preview;

pub static GLOBAL: Lazy<Global> = Lazy::new(|| Global::from(&POSTS) );

#[derive(Serialize)]
pub struct Global {
    pub title: String,
    pub subtitle: Option<String>,
    pub copyright: Option<String>,
    pub footnote: Option<String>,
    pub categories_href: String,
    pub about_href: String,
}

impl Global {
    fn from(posts: &Posts) -> Self {
        let site = &CONFIG.site;
        Self {
            title: site.title.clone(),
            subtitle: site.subtitle.clone(),
            copyright: site.copyright.clone(),
            footnote: site.footnote.clone(),
            categories_href: posts.categories_href(),
            about_href: posts.about_href(),
        }
    }
}
