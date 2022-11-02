use std::path::Path;

use anyhow::Error;

use crate::page::Article;

pub fn gen_page(file: &Path) -> Result<Option<(Article, String)>, Error> {
    if !file.exists() { return Ok(None); }

    let article = Article::from(file)?;


    // is single page
    Ok(Some((article, "".into())))
}

pub fn gen_page_write(file: &Path) -> Result<(), Error> {
    todo!()
}
