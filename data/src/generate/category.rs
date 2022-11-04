use std::path::Path;

use anyhow::Error;
use chrono::NaiveDate;

use config::{parent, workspace};
use render::Template;

use crate::generate::page;
use crate::generate::page::page_url;
use crate::page::{Article, Category};
use crate::template::items::{ArticleItem, CategoryTpl};
use crate::template::page::PageTpl;

pub fn gen(path: &Path) -> Result<String, Error> {
    let (category, articles) = create(path)?;
    let value = CategoryTpl::from(articles.as_slice(), &category);
    Template::Category.render(value)
}

pub fn write(path: &Path) -> Result<(), Error> {
    let (category, articles) = create(path)?;
    for (name, article) in &articles {
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        page::write(file_stem, article, Some(&category))?;
    }

    let value = CategoryTpl::from(articles.as_slice(), &category);
    let target = parent().join(&workspace().publish.self_dir)
        .join(format!("{}.html", category.name));
    Template::Category.render_write(value, &target)
}

fn create(path: &Path) -> Result<(Category, Vec<(String, Article)>), Error> {
    let category = Category::from(&path)?;
    let mut articles = vec![];
    for p in path.read_dir()? {
        let file = p?.path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.ends_with(".adoc") && name != "index.adoc" {
            articles.push((name.to_string(), Article::from(&file)?));
        }
    }
    Ok((category, articles))
}
