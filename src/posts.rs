use std::path::PathBuf;

use anyhow::Error;
use chrono::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_derive::Deserialize;

use crate::config::CONFIG;
use crate::convert::Template;
use crate::posts::scan::scan_files;
use crate::posts::tpl::article;
use crate::posts::tpl::article::ArticleTpl;
use crate::posts::tpl::preview::{CategoriesTpl, CategoryTpl};

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
    // todo build index

    // render
    let publish = &CONFIG.workspace.publish;
    let tpl = CategoriesTpl::from(&posts.categories_index);
    let target = publish.join(format!("/categories.html"));
    Template::Categories.render_write(&tpl, &target)?;
    for c in &posts.categories {
        let tpl = CategoryTpl::from(&c.index);
        let target = publish.join(c.href());
        Template::Category.render_write(&tpl, &target)?;
        for a in &c.files {
            let url_name = REG.replace(&a.name, ".html");
            let path = CONFIG.site.slug.as_ref()
                .map(|v| format!("{}/{}", v, url_name))
                .unwrap_or_else(|| url_name.to_string());

            let target = publish.join(path);
            let tpl = article::build_tpl(&a.path, c)?;
            Template::Article.render_write(&tpl, &target)?;
        }
    }

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

impl TextFile {
    pub fn href(&self) -> String {
        let url_name = REG.replace(&self.name, ".html");
        let href = CONFIG.site.slug.as_ref()
            .map(|v| format!("/{}/{}", v, url_name))
            .unwrap_or_else(|| format!("/{}", url_name));
        href
    }
}

pub struct Category {
    pub name: String,
    pub files: Vec<TextFile>,
    pub config: CategoryConfig,
    pub index: Generated,
}

impl Category {
    fn href(&self) -> String {
        let l = self.config.alias_name.as_ref().unwrap_or_else(|| &self.name);
        format!("/categories/{}.html", l)
    }

    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty() && self.name.trim() != "index" &&
        self.config.alias_name.as_ref()
            .map(|v| !v.trim().is_empty() && v.trim() != "index")
            .unwrap_or(true)
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
    pub href: String,
    pub pin: bool,
    pub created_at: NaiveDate,
    pub summary: Option<String>,
    pub category: String,
    pub category_href: String,
}

static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(adoc)$").unwrap());

#[cfg(test)]
mod test {
    use crate::posts::generate_site;

    #[test]
    fn test() {
        generate_site().unwrap();
    }
}