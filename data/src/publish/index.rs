use anyhow::Error;

use config::{parent, workspace};
use render::Template;

use crate::publish::{categories, category};
use crate::template::category::{CategoryTpl, IndexTpl};

pub fn gen() -> Result<String, Error> {
    create(false).map(|v| v.unwrap())
}

pub fn write() -> Result<(), Error> {
    create(true).map(|_| ())
}

fn create(write: bool) -> Result<Option<String>, Error> {
    let mut vec = vec![];
    let (_, paths) = categories::create()?;
    for path in &paths {
        vec.push(category::create(path)?);
    }

    let mut article_items = vec![];
    for (category, articles) in &vec {
        let tpl = CategoryTpl::from(&articles, &category);
        article_items.extend(tpl.items);
    }

    let index_tpl = IndexTpl::from(article_items);
    let target = parent().join(&workspace().publish.self_dir).join("index.html");
    match write {
        true => Template::Index.render_write(index_tpl, &target).map(|_| None),
        _ => Template::Index.render(index_tpl).map(|v| Some(v)),
    }
}
