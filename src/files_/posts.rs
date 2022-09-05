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


