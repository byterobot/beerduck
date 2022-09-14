use std::collections::LinkedList;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

use anyhow::Error;
use chrono::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_derive::Deserialize;

use crate::config::CONFIG;
use crate::convert::Template;
use crate::posts::page::Page;
use crate::posts::scan::scan_files;
use crate::posts::tpl::about::AboutTpl;
use crate::posts::tpl::{about, article};
use crate::posts::tpl::article::ArticleTpl;
use crate::posts::tpl::preview::{CategoriesTpl, CategoryTpl};

mod scan;
mod gen;
mod render;
mod page;
mod tpl;

pub static POSTS: Lazy<Posts> = Lazy::new(|| load_files().unwrap());

pub fn generate_site() -> Result<(), Error> {
    let posts = POSTS.deref();
    let publish = &CONFIG.workspace.publish;

    for c in &posts.categories {
        let tpl = CategoryTpl::from(&c.index);
        let target = publish.join(c.href_relative());
        Template::Category.render_write(&tpl, &target)?;
        for a in &c.files {
            let page = Page::from(&a.path)?;
            copy_files(&page.images)?;
            let tpl = article::build_tpl(page, c)?;
            let target = publish.join(a.href_relative());
            Template::Article.render_write(&tpl, &target)?;
        }
    }

    let tpl = CategoriesTpl::from(&posts.categories_index);
    let target = publish.join(posts.categories_href_relative());
    Template::Categories.render_write(&tpl, &target)?;

    let tpl = CategoriesTpl::from(&posts.index);
    let target = publish.join(posts.index_href_relative());
    Template::Index.render_write(&tpl, &target)?;

    let tpl = about::build_tpl(&posts.about.path)?;
    let target = publish.join(posts.about_href_relative());
    Template::About.render_write(&tpl, &target)?;

    Ok(())
}

fn load_files() -> Result<Posts, Error> {
    // scan locations
    let mut posts = scan_files(&CONFIG.workspace)?;

    // build navigation
    posts.categories_index = gen::gen_categories(posts.categories.as_slice())?;
    for c in posts.categories.iter_mut() {
        c.index = gen::gen_category(c)?;
    }

    // build index
    posts.index = gen::gen_index(posts.categories.as_slice())?;

    Ok(posts)
}

fn copy_files(files: &LinkedList<(String, String)>) -> Result<(), Error> {
    let static_ = &CONFIG.workspace.static_;
    let publish = &CONFIG.workspace.publish;
    for (src, target) in files {
        let s = static_.join(src.replacen("/", "", 1));
        let t = publish.join(target.replacen("/", "", 1));
        // println!("{:?}", s);
        // println!("{:?}", t.parent().unwrap());
        fs::create_dir_all(&t.parent().unwrap())?;
        fs::copy(&s, &t).map_err(|e| {
            println!("error: {}, path: {}", e, s.to_str().unwrap())
        });
    }

    Ok(())
}


pub struct Posts {
    index: Generated,
    categories_index: Generated,
    categories: Vec<Category>,
    about: TextFile,
}

impl Posts {
    pub fn index_href(&self) -> String {
        "/index.html".into()
    }
    pub fn index_href_relative(&self) -> String {
        "index.html".into()
    }
    pub fn categories_href(&self) -> String {
        "/categories.html".into()
    }
    pub fn categories_href_relative(&self) -> String {
        "categories.html".into()
    }
    pub fn about_href(&self) -> String {
        "/about.html".into()
    }
    pub fn about_href_relative(&self) -> String {
        "about.html".into()
    }
}


pub struct TextFile {
    pub name: String,
    pub path: PathBuf,
}

impl TextFile {
    pub fn href(&self) -> String {
        format!("/{}", self.href_relative())
    }
    pub fn href_relative(&self) -> String {
        let url_name = REG.replace(&self.name, ".html");
        let href = CONFIG.site.slug.as_ref()
            .map(|v| format!("{}/{}", v, url_name))
            .unwrap_or_else(|| format!("{}", url_name));
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
        format!("/{}", self.href_relative())
    }

    fn href_relative(&self) -> String {
        let l = self.config.alias_name.as_ref().unwrap_or_else(|| &self.name);
        format!("categories/{}.html", l)
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

#[derive(Default)]
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