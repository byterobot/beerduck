use std::path::Path;

use anyhow::Error;
use chrono::{Date, Utc};

use crate::files::category::Category;
use crate::files::page::Page;

pub struct Posts {
    home: String, // 生成
    category: String, // 生成
    about: Option<Page>,
    categories: Vec<Category>,
}

pub fn generate_write() -> Result<(), Error> {
    // 生成列表, 生成首页, about页


    todo!()
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
