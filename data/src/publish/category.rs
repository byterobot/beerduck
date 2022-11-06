use std::path::{Path, PathBuf};

use anyhow::Error;
use chrono::NaiveDate;

use config::{parent, workspace};
use render::Template;

use crate::page::{Article, Category};
use crate::publish::page;
use crate::template::category::{ArticleItem, CategoryTpl};
use crate::template::page::PageTpl;

pub fn gen(path: &Path) -> Result<String, Error> {
    let (category, articles) = create(path)?;
    let value = CategoryTpl::from(&articles, &category);
    Template::Category.render(value)
}

pub fn write(path: &Path) -> Result<(), Error> {
    let (category, articles) = create(path)?;
    for (name, article) in &articles {
        page::write(&name, article, Some(&category))?;
    }

    let value = CategoryTpl::from(articles.as_slice(), &category);
    let target = parent()
        .join(&workspace().publish.categories_dir)
        .join(format!("{}.html", category.name));
    Template::Category.render_write(value, &target)
}

pub fn create(path: &Path) -> Result<(Category, Vec<(String, Article)>), Error> {
    let category = Category::from(&path)?;
    let mut articles = vec![];
    for p in path.read_dir()? {
        let file = p?.path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.ends_with(".adoc") && name != "index.adoc" {
            let name_stem = file.file_stem().unwrap().to_str().unwrap().to_string();
            articles.push((name_stem, Article::from(&file)?));
        }
    }
    Ok((category, articles))
}

pub fn files(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut files = vec![];
    for p in path.read_dir()? {
        let file = p?.path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.ends_with(".adoc") && name != "index.adoc" {
            files.push(file);
        }
    }
    Ok(files)
}