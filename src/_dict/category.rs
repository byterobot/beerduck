use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Error;
use regex::Regex;
use serde_derive::Deserialize;

use crate::config::{CONFIG, Config};
use crate::config::workspace::Workspace;

// category.toml
#[derive(Default, Deserialize)]
struct CategoryConfig {
    position: u16, // 在 category 目录中的排序
    date_asc: bool, //
    // 以下两者 nav 优先
    nav: Option<String>, // example.adoc
    new_name: Option<String>, // <new-name>.html
}

pub struct CategoryFolder {
    path: PathBuf,
    category_name: String,
    config: CategoryConfig,
    adoc_files: Vec<String>, // [a.adoc, b.adoc, ...]
}

#[derive(Debug)]
pub struct Category {
    pub category_name: String,
    pub path: PathBuf,
    pub position: u16, // 在 category 目录中的排序
    pub date_asc: bool, // true -> asc by date; false -> desc by date

    pub adoc_name: Option<String>, // nav.adoc
    pub adoc_absolute: Option<PathBuf>,

    pub html_name: String, // nav.html or category_name.html or new_name.html
    pub html_relative: PathBuf,
    pub html_absolute: PathBuf,

    pub adoc_files: Vec<String>,
}

impl Category {
    pub fn from(mut c: CategoryFolder, config: &Config) -> Self {
        let curr = c.path.as_path();
        let adoc_absolute = c.config.nav.as_ref().map(|v| curr.join(v));
        let reg = Regex::new(r"\.(adoc)$").unwrap();
        let html_name = match &c.config.nav {
            // Some(name) => name.replace(r"\.(adoc)$", ".html"),
            Some(name) => reg.replace(name, ".html").to_string(),
            _ => c.config.new_name.unwrap_or_else(|| format!("{}.html", c.category_name)),
        };

        let html_relative = config.site.slug.as_ref()
            .map(|s| format!("{}/{}", s, html_name))
            .unwrap_or_else(|| html_name.clone());

        if let Some(nav) = &c.config.nav {
            if let Some(pos) = c.adoc_files.iter().position(|name| name.as_str() == nav.as_str()) {
                c.adoc_files.remove(pos);
            }
        }

        Category {
            category_name: c.category_name,
            path: c.path,
            position: c.config.position,
            date_asc: c.config.date_asc,
            adoc_name: c.config.nav,
            adoc_absolute,
            html_name,
            html_relative: PathBuf::from(&html_relative),
            html_absolute: config.workspace.publish.join(html_relative),
            adoc_files: c.adoc_files
        }
    }
}

pub fn build_category_map(config: &Config) -> Result<HashMap<String, Category>, Error> {
    let mut map = HashMap::new();
    for dir in config.workspace.posts.read_dir()? {
        let path = dir?.path();
        if path.is_dir() {
            let category = Category::from(read_files(&path)?, config);
            map.insert(category.category_name.clone(), category);
        }
    }
    Ok(map)
}

fn read_files(path: &Path) -> Result<CategoryFolder, Error> {
    let mut category = CategoryFolder {
        // 文件夹名必然存在
        path: path.to_path_buf(),
        category_name: path.file_name().unwrap().to_str().unwrap().to_string(),
        config: Default::default(),
        adoc_files: vec![],
    };
    for dir in path.read_dir()? {
        let dir = dir?;
        if let Some(v) = dir.file_name().to_str() {
            if v.ends_with(".adoc") {
                category.adoc_files.push(v.to_string());
            } else if v == "category.toml" {
                let text = fs::read_to_string(dir.path())?;
                category.config = toml::from_str::<CategoryConfig>(&text)?;
            }
        }
    }
    Ok(category)
}
