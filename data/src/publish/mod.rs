use std::fs;
use anyhow::Error;
use fs_extra::dir::CopyOptions;
use log::debug;
use config::{parent, workspace};
use crate::appoint::Appoint;
use crate::page::{Article, Category};

pub mod page;
pub mod category;
pub mod categories;
pub mod index;
pub mod about;

pub fn get(point: &Appoint) -> Result<String, Error> {
    match point {
        Appoint::Page(category, file) => {
            let c = match category {
                Some(c) => Some(Category::from(&parent().join(c))?),
                _ => None,
            };

            let name = category.as_ref().map(|v| format!("{}/", v)).unwrap_or_default();
            let path = parent().join(format!("{}{}.adoc", name, file));
            page::gen(&Article::from(&path)?, c.as_ref())
        }
        Appoint::Category(name) => category::gen(&parent().join(name)),
        Appoint::Categories => categories::gen(),
        Appoint::Index => index::gen(),
        Appoint::Unknown(url) => {
            debug!("Not Found: {}", url);
            Ok("404 Not Found".into())
        }
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