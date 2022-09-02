use std::fs;
use std::path::Path;

use anyhow::{anyhow, Error};
use serde_derive::Deserialize;

use crate::files::page::Page;

pub struct Category {
    name: String, // category name
    cfg: Config, // category.toml
    page: Option<Page>,
    pages: Vec<Page>
}

#[derive(Default, Deserialize)]
pub struct Config {
    permalink: Option<String>,
    order: bool, // true -> asc by date; false -> desc by date
    position: u16, // 在 category 目录中的排序
}

impl Category {
    pub fn create(path: &Path) -> Result<Self, Error> {
        let mut category = Category {
            name: category_name(path)?,
            cfg: deserialize_config(path)?,
            page: build_index(path)?,
            pages: load_pages(path)?,
        };
        category.pages.sort_by(|a, b| match category.cfg.order {
            true => a.date.cmp(&b.date),
            _ => b.date.cmp(&a.date),
        });

        Ok(category)
    }

    pub fn render(&self) -> Result<(), Error> {
        // 此 category 下的page 列表
        todo!()
    }
}

// 缺失的索引页在模板中构建
fn build_index(path: &Path) -> Result<Option<Page>, Error> {
    let file = path.join("_index.adoc");
    match file.exists() {
        true => Ok(Some(Page::create(&file)?)),
        _ => Ok(None),
    }
}

fn load_pages(path: &Path) -> Result<Vec<Page>, Error> {
    let mut pages = vec![];
    for dir in path.read_dir()? {
        let dir = dir?;
        if let Some(name) = dir.file_name().to_str() {
            if name.ends_with(".adoc") && name != "index.adoc" && name != "_index.adoc" {
                pages.push(Page::create(&dir.path())?);
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
    let path = path.join("_category.toml");
    match fs::read_to_string(&path) {
        Ok(v) => Ok(toml::from_str(&v)?),
        Err(_) => Ok(Config::default())
    }
}
