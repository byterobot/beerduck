use std::path::Path;

use anyhow::Error;
use chrono::NaiveDate;
use render::Template;
use crate::generate::page::page_url;

use crate::page::{Article, Category};
use crate::template::items::{Item, ItemsTpl};
use crate::template::page::PageTpl;

pub fn gen(path: &Path) -> Result<String, Error> {
    let category = Category::from(&path)?;
    let mut articles = vec![];
    for p in path.read_dir()? {
        let file = p?.path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.ends_with(".adoc") && name != "index.adoc" {
            articles.push((name.to_string(), Article::from(&file)?));
        }
    }
    Template::Category.render(ItemsTpl::from(articles.as_slice(), &category))
}

pub fn gen_write(path: &Path) -> Result<(), Error> {
    todo!()
}