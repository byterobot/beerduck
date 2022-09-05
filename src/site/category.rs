use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Error;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

use crate::config::CONFIG;

// category.toml
#[derive(Default, Deserialize)]
struct CategoryConfig {
    position: u16, // 在 category 目录中的排序
    order: bool, // true -> asc by date; false -> desc by date
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

pub struct Category {
    pub category_name: String,
    pub path: PathBuf,
    pub position: u16, // 在 category 目录中的排序
    pub order: bool, // true -> asc by date; false -> desc by date
    pub adoc_name: Option<String>, // nav.adoc
    pub adoc_absolute: Option<PathBuf>,
    pub html_name: String, // nav.html or category_name.html or new_name.html
    pub html_relative: PathBuf,
    pub html_absolute: PathBuf,
    pub adoc_files: Vec<String>,
}

impl Category {
    pub fn from(c: CategoryFolder) -> Self {
        let curr = c.path.as_path();
        let adoc_absolute = c.config.nav.as_ref().map(|v| curr.join(v));
        let html_name = match &c.config.nav {
            Some(name) => name.replace(r"\.(adoc)$", ".html"),
            _ => c.config.new_name.unwrap_or_else(|| format!("{}.html", c.category_name)),
        };

        let html_relative = CONFIG.site.slug.as_ref()
            .map(|s| format!("{}/{}", s, html_name))
            .unwrap_or_else(|| html_name.clone());

        Category {
            category_name: c.category_name,
            path: c.path,
            position: c.config.position,
            order: c.config.order,
            adoc_name: c.config.nav,
            adoc_absolute,
            html_name,
            html_relative: PathBuf::from(&html_relative),
            html_absolute: CONFIG.dir.publish.join(html_relative),
            adoc_files: vec![]
        }
    }
}

// example.adoc -> category name
pub static CATEGORY: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (k, v) in CATEGORIES.iter() {
        for a in &v.adoc_files {
            map.insert(a.clone(), k.clone());
        }
    }
    map
});

// category name -> category
pub static CATEGORIES: Lazy<HashMap<String, Category>> = Lazy::new(|| {
    // 按规则生成导航, 最后替换主页
    scan_category().expect("scan categories error")
});

fn scan_category() -> Result<HashMap<String, Category>, Error> {
    let d = &CONFIG.dir;
    let mut map = HashMap::new();
    for dir in d.workspace.read_dir()? {
        let path = dir?.path();
        if path.is_dir() {
            let category = Category::from(read_files(&path)?);
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
