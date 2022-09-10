use std::fs;
use std::path::Path;

use anyhow::{anyhow, Error};

use crate::config::workspace::Workspace;
use crate::posts::{Category, CategoryConfig, Posts, TextFile};

pub fn scan_files(workspace: &Workspace) -> Result<Posts, Error> {
    let mut about = None;
    let mut categories = vec![];
    for dir in workspace.posts.read_dir()? {
        let path = dir?.path();
        if path.is_file() {
            if about.is_none() && path.ends_with("about.adoc") {
                about = Some(TextFile { name: "about.adoc".to_string(), path });
                continue;
            }
        } else if path.is_dir() {
            categories.push(scan_category(&path)?);
        }
    }

    let p = Posts {
        index: Default::default(),
        categories_index: Default::default(),
        categories,
        about: about.ok_or_else(|| anyhow!("`about.adoc` file not found"))?
    };
    Ok(p)
}

fn scan_category(path: &Path) -> Result<Category, Error> {
    let mut config = CategoryConfig::default();
    let mut files = vec![];
    for dir in path.read_dir()? {
        let dir = dir?;
        if let Some(name) = dir.file_name().to_str() {
            if name == "index.adoc" {
                return Err(anyhow!("the file name should not be `index.adoc`"));
            }
            if name.ends_with(".adoc") {
                let file = TextFile { name: name.into(), path: dir.path() };
                files.push(file);
            } else if name == "config.toml" {
                // let t = fs::read_to_string(dir.path())?;
                config = toml::from_str(&fs::read_to_string(dir.path())?)?;
            }
        }
    }

    let c = Category {
        name: path.file_name().unwrap().to_str().unwrap().to_string(),
        files,
        config,
        index: Default::default()
    };

    if !c.is_valid() {
        Err(anyhow!("invalid category info"))?;
    }

    Ok(c)
}













