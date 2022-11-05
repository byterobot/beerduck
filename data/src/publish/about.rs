use anyhow::Error;

use config::{parent, site, workspace};

use crate::page::Article;
use crate::publish::page;

pub fn gen() -> Result<String, Error> {
    let file = parent().join(&workspace().posts).join(format!("{}.adoc", site().about));
    page::gen(&Article::from(&file)?, None)
}

pub fn write() -> Result<(), Error> {
    let file = parent().join(&workspace().posts).join(format!("{}.adoc", site().about));
    page::write(&site().about, &Article::from(&file)?, None)
}
