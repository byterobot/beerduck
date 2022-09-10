use std::path::PathBuf;

use anyhow::Error;
use chrono::NaiveDate;
use serde_derive::Deserialize;

use crate::config::CONFIG;
use crate::posts::scan::scan_files;

mod scan;
mod gen;
mod render;
mod page;
mod tpl;

pub fn generate_site() -> Result<(), Error> {
    // scan locations
    let mut posts = scan_files(&CONFIG.workspace)?;

    // build navigation
    posts.categories_index = gen::gen_categories(posts.categories.as_slice())?;
    for c in posts.categories.iter_mut() {
        c.index = gen::gen_category(c)?;
    }

    // render

    Ok(())
}

pub struct Posts {
    index: Generated,
    categories_index: Generated,
    categories: Vec<Category>,
    about: TextFile,
}

pub struct TextFile {
    pub name: String,
    pub path: PathBuf,
}

pub struct Category {
    pub name: String,
    pub files: Vec<TextFile>,
    pub config: CategoryConfig,
    pub index: Generated,
}

impl Category {
    pub fn link(&self) -> String {
        let l = self.config.alias_name.as_ref().unwrap_or_else(|| &self.name);
        format!("categories/{}.html", l)
    }
}

// category.toml
#[derive(Default, Deserialize)]
pub struct CategoryConfig {
    pub position: u16, // 在 category 目录中的排序
    pub date_asc: bool,
    pub pin: Option<String>, // example.adoc
    pub alias_name: Option<String>, // <alias_name>.html
}

#[derive(Default)]
pub struct Generated {
    pub title: String,
    pub items: Vec<Preview>,
}

pub struct Preview {
    pub title: String,
    pub pin: bool,
    pub created_at: NaiveDate,
    pub summary: Option<String>,
    pub link_name: String,
    pub category: String,
    pub category_link_name: String,
}
