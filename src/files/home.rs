use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Error;

use crate::files::category::Category;

/// 只支持一层分类
pub struct Home {
    // pub index: Option<PathBuf>, // 不能自定义
    // pub category: Option<PathBuf>, // category list adoc
    pub categories: Vec<Category>,
    pub single_page: Option<PathBuf>, // 如果存在多个, 只选取文件系统列表中的第一个
}

impl Home {
    pub fn from(posts: &Path) -> Self {
        match read_dir(posts) {
            Ok(v) => v,
            Err(e) => panic!("Create home `{:?}` failed: {}", posts, e),
        }
    }
}

fn read_dir(posts: &Path) -> Result<Home, Error> {
    let mut single_page = None;
    for dir in posts.read_dir()? {
        let dir = dir?;
        if let Some(v) = dir.file_name().to_str() {
            if v.ends_with(".adoc") && dir.path().is_file() {
                single_page = Some(dir.path());
                break;
            }
        }
    }

    let mut categories = vec![];
    for dir in posts.read_dir()? {
        let category = Category::from(&dir?.path());
        if !category.vec.is_empty() {
            categories.push(category);
        }
    }

    Ok(Home { categories, single_page, })
}
