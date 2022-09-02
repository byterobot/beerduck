use std::path::Path;

use anyhow::Error;
use chrono::{Date, Utc};

use crate::files::category::Category;
use crate::files::page::Page;

pub fn render() {
    // let pages: HashMap<PathBuf, Page> = HashMap::new(); // articles
    // category list
    // category pages list
    // index
    // single page, (not in others) // 如果存在多个, 只选取文件系统列表中的第一个
}

pub struct Posts {
    home: String, // 生成
    category: String, // 生成
    about: Option<Page>,
    categories: Vec<Category>,
}

impl Posts {
    pub fn create(path: &Path) -> Result<Self, Error> {
        todo!()
    }

    pub fn render(&self) -> Result<(), Error> {
        // 首页和 所有 category 列表
        todo!()
    }
}







/*
fn read_dir(posts: &Path) -> Result<Posts, Error> {
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
        categories.push(category);
        // if !category.vec.is_empty() {
        //     categories.push(category);
        // }
    }

    todo!()
    // Ok(Posts { categories, single_page, })
}
*/
