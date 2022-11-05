use std::path::{Path, PathBuf};

use anyhow::Error;

use config::{parent, workspace};
use render::Template;

use crate::page::Category;
use crate::publish::category;
use crate::template::category::CategoriesTpl;

pub fn gen() -> Result<String, Error> {
    let (categories, _) = create()?;
    let value = CategoriesTpl::from(&categories);
    Template::Categories.render(value)
}

pub fn write() -> Result<(), Error> {
    let (categories, paths) = create()?;
    for path in paths.as_slice() {
        category::write(path)?;
    }

    let value = CategoriesTpl::from(&categories);
    let target = parent().join(&workspace().publish.categories_index);
    Template::Categories.render_write(value, &target)
}

pub fn create() -> Result<(Vec<Category>, Vec<PathBuf>), Error> {
    let dir = parent().join(&workspace().posts).read_dir()?
        .into_iter()
        .filter(|v| v.is_ok() && v.as_ref().unwrap().path().is_dir())
        .map(|v| v.unwrap().path())
        .collect::<Vec<PathBuf>>();

    let mut categories = vec![];
    for path in &dir {
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        if file_stem != "static" && file_stem != "index" {
            categories.push(Category::from(&path)?);
        }
    }
    Ok((categories, dir))
}
