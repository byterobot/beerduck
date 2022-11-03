use std::path::Path;

use anyhow::Error;
use chrono::NaiveDate;

use config::{parent, workspace};
use render::Template;

use crate::generate::page;
use crate::generate::page::page_url;
use crate::page::{Article, Category};
use crate::template::items::{Item, ItemsTpl};
use crate::template::page::PageTpl;

pub fn gen(path: &Path) -> Result<String, Error> {
    Ok(gen_inner(path, false)?.unwrap())
}

pub fn gen_write(path: &Path) -> Result<(), Error> {
    gen_inner(path, true)?;
    Ok(())
}

fn gen_inner(path: &Path, write: bool) -> Result<Option<String>, Error> {
    let category = Category::from(&path)?;
    let mut articles = vec![];
    for p in path.read_dir()? {
        let file = p?.path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.ends_with(".adoc") && name != "index.adoc" {
            let article = Article::from(&file)?;
            if write {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                page::gen_write(file_stem, &article, Some(&category))?;
            }
            articles.push((name.to_string(), article));
        }
    }

    let value = ItemsTpl::from(articles.as_slice(), &category);
    if write {
        let target = parent().join(&workspace().publish.self_dir)
            .join(format!("{}.html", category.name));
        Template::Category.render_write(value, &target)?;
        Ok(None)
    } else {
        Ok(Some(Template::Category.render(value)?))
    }
}
