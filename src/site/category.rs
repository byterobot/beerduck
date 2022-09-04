use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Error;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

use crate::config::CONFIG;

// category.toml
#[derive(Default, Deserialize)]
pub struct CategoryConfig {
    position: u16, // 在 category 目录中的排序
    order: bool, // true -> asc by date; false -> desc by date
    // 以下两者 nav 优先
    nav: Option<String>, // example.adoc
    new_name: Option<String>, // <new-name>.html
}

pub struct Category {
    category_name: String,
    config: CategoryConfig,
    adoc_files: Vec<String>, // [a.adoc, b.adoc, ...]
}

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
            let category = read_files(&path)?;
            map.insert(category.category_name.clone(), category);
        }
    }
    Ok(map)
}

fn read_files(path: &Path) -> Result<Category, Error> {
    let mut category = Category {
        // 文件夹名必然存在
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
