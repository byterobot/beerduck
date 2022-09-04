use std::fs;
use std::path::Path;

use anyhow::{anyhow, Error};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde_derive::Deserialize;

use crate::config::CONFIG;
use crate::files::page;
use crate::files::page::Page;
use crate::files::render::Template;
use crate::files::template::{CategoryTpl, PageTpl};

pub struct Category {
    pub name: String, // category name
    pub nav_page: Page,
    pub pages: Vec<Page>,
    pub position: u16, // 在 category 目录中的排序
    pub order: bool, // true -> asc by date; false -> desc by date
}

// category.toml
#[derive(Default, Deserialize)]
pub struct Config {
    position: u16, // 在 category 目录中的排序
    order: bool, // true -> asc by date; false -> desc by date
    // 以下两者 index 优先
    index: Option<String>, // example.adoc
    permalink: Option<String>, // new-name.html
}

// render category
pub fn render(folder: &Path) -> Result<Category, Error> {
    let cfg = deserialize_config(folder)?;
    let name = category_name(folder)?;
    let mut pages = render_pages(folder)?;
    pages.sort_by(|a, b| match cfg.order {
        true => a.created_at.cmp(&b.created_at),
        _ => b.created_at.cmp(&a.created_at),
    });

    let page = match &cfg.index {
        Some(v) => pick_index(v.as_ref(), &mut pages)?,
        _ => build_index(&name, cfg.permalink.as_ref(), pages.as_slice())?,
    };

    let mut category = Category {
        name,
        nav_page: page,
        pages,
        position: cfg.position,
        order: cfg.order,
    };
    Ok(category)
}

fn pick_index(index_file: &str, pages: &mut Vec<Page>) -> Result<Page, Error> {
    let permalink = index_file.replace(r"\.(adoc)$", ".html");
    let item = pages.iter().enumerate()
        .find(|(a, b)| &b.permalink == &permalink)
        .ok_or_else(|| anyhow!("no page for: {}", permalink))?;
    Ok(pages.remove(item.0))
}

fn build_index(name: &String, permalink: Option<&String>, pages: &[Page]) -> Result<Page, Error> {
    let site = &CONFIG.site;
    let pages: Vec<PageTpl> = pages.iter().map(|v| PageTpl::from(v)).collect();
    let tpl = CategoryTpl {
        site,
        title: name,
        lang: &site.lang,
        pages: pages.as_slice(),
    };

    let page = Page {
        autogen: true,
        permalink: permalink.map(|v| v.clone()).unwrap_or_else(|| format!("{}.html", name)),
        title: name.clone(),
        author: site.author.clone(),
        full_html: Template::Category.render(&tpl)?,
        lang: site.lang.clone(),
        keywords: None,
        description: None,
        summary: None,
        created_at: Default::default(),
        created_at_num: (0, "".to_string(), "".to_string()),
        updated_at: None,
        nav_html: None,
        content_html: Default::default()
    };

    Ok(page)
}

fn render_pages(folder: &Path) -> Result<Vec<Page>, Error> {
    let mut pages = vec![];
    for dir in folder.read_dir()? {
        let dir = dir?;
        if let Some(name) = dir.file_name().to_str() {
            if name.ends_with(".adoc") {
                pages.push(page::render(&dir.path())?);
            }
        }
    }
    Ok(pages)
}

fn category_name(path: &Path) -> Result<String, Error> {
    path.file_name().ok_or_else(|| anyhow!("Not a folder: {:?}", path))
        .map(|v| v.to_string_lossy().to_string())

}

fn deserialize_config(path: &Path) -> Result<Config, Error> {
    let path = path.join("category.toml");
    match fs::read_to_string(&path) {
        Ok(v) => Ok(toml::from_str(&v)?),
        Err(_) => Ok(Config::default())
    }
}
