use std::fs;
use anyhow::{anyhow, Error};
use fs_extra::dir::CopyOptions;
use log::debug;
use config::{parent, workspace};
use crate::Endpoint;
use crate::page::{Article, Category};

pub mod page;
pub mod category;
pub mod categories;
pub mod index;
pub mod about;

pub fn get(point: &Endpoint) -> Result<String, Error> {
    match point {
        Endpoint::Page(category, file) => {
            let path = parent().join(&workspace().posts)
                .join(format!("{}/{}.adoc", category, file));
            let category = Category::from(&parent().join(category))?;
            page::gen(&Article::from(&path)?, Some(&category))
        }
        Endpoint::About => about::gen(),
        Endpoint::Category(name) => category::gen(&parent().join(name)),
        Endpoint::Categories => categories::gen(),
        Endpoint::Index => index::gen(),
        Endpoint::Unknown(url_path) => Err(anyhow!("404 Not Found {}", url_path)),
    }
}

pub fn publish() -> Result<(), Error> {
    copy_static()?;
    categories::write()?;
    about::write()?;
    index::write()?;
    Ok(())
}

fn copy_static() -> Result<(), Error> {
    let from = parent().join(&workspace().theme.static_.self_dir);
    let to = parent().join(&workspace().publish.self_dir);
    fs::create_dir_all(&to)?;
    let mut opts = CopyOptions::new();
    opts.overwrite = true;
    fs_extra::dir::copy(from, to, &opts)?;
    Ok(())
}