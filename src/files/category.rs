use std::path::{Path, PathBuf};

use anyhow::{anyhow, Error};
use chrono::{Date, Utc};

use crate::config::CONFIG;
use crate::files::page::Page;

pub struct Category {
    title: Option<String>, // 自定义的可能会有title, 自动生成的一般是 category name.
    name: String,
    path: String,
    html: String,
    pages: Vec<Page>
}

impl Category {
    pub fn create(path: &Path) -> Result<Self, Error> {
        todo!()
    }
}


/*
fn read_dir(path: &Path) -> Result<Info, Error> {
    let (name, new_path) = read_name(path)?;
    let mut index = None;
    let mut vec = vec![];
    for dir in path.read_dir()? {
        let dir = dir?;
        if let Some(name) = dir.file_name().to_str() {
            if index == None && name == "_index.adoc" {
                index = Some(dir.path());
            } else if name.ends_with(".adoc") {
                vec.push(dir.path());
            }
        }
    }

    Ok(Info { name, new_path, index, vec })
}*/

fn read_name(path: &Path) -> Result<(String, String), Error> {
    let category_name = path
        .file_name().ok_or_else(|| anyhow!("Invalid category name"))?
        .to_str().ok_or_else(|| anyhow!("Invalid category name"))?
        .to_string();

    for dir in path.read_dir()? {
        let a = dir?;
        if let Some(name) = a.file_name().to_str() {
            if name.ends_with(".url") && name.starts_with("_") {
                let new_name = Path::new(name)
                    .file_stem().ok_or_else(|| anyhow!("Invalid new path"))?
                    .to_str().ok_or_else(|| anyhow!("Invalid new path"))?
                    .replacen('_', "", 1);
                return Ok((category_name, new_name));
            }
        }
    }

    let new_name = category_name.clone();
    Ok((category_name, new_name))
}
/*
pub struct CategoryOutput {
    // category name
    pub name: String,
    // (category path, category index html content)
    pub index: (String, String),
    // (url name, article html content)
    pub vec: Vec<(String, String)>,
}*/