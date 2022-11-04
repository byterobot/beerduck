use anyhow::Error;

use config::{parent, site};

use crate::generate::page;
use crate::page::Article;

pub fn gen() -> Result<String, Error> {
    let file = parent().join(format!("{}.adoc", site().about));
    page::gen(&Article::from(&file)?, None)
}

pub fn write() -> Result<(), Error> {
    let file = parent().join(format!("{}.adoc", site().about));
    let file_stem = file.file_stem().unwrap().to_str().unwrap();
    page::write(file_stem, &Article::from(&file)?, None)
}
