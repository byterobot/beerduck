use std::path::Path;

use anyhow::Error;

// dev_mode would not write to disk

pub fn gen_page(file: &Path) -> Result<String, Error> {
    // is single page
    todo!()
}

pub fn gen_category(folder: &Path) -> Result<String, Error> {
    todo!()
}

pub fn gen_categories() -> Result<String, Error> {
    todo!()
}

pub fn gen_index() -> Result<(String), Error> {
    todo!()
}