pub mod page;
pub mod category;
pub mod categories;
pub mod index;

use std::fs;
use std::path::Path;

use anyhow::Error;
use crate::page::Article;

// dev_mode would not write to disk



pub fn gen_category(folder: &Path) -> Result<String, Error> {
    todo!()
}

pub fn gen_categories() -> Result<String, Error> {
    todo!()
}

pub fn gen_index() -> Result<String, Error> {
    todo!()
}