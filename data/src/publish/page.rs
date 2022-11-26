use std::fs;
use std::path::Path;

use anyhow::Error;
use chrono::{Datelike, NaiveDate};

use config::{to_relative, parent, site, workspace};
use render::Template;

use crate::page::{Article, Category};
use crate::template::page::PageTpl;
use crate::template::page_url;

pub fn gen<'a>(article: &'a Article, category: Option<&'a Category>) -> Result<String, Error> {
    match category {
        Some(c) => Template::Article.render(PageTpl::from(article, c)),
        _ => Template::About.render(PageTpl::single(article)),
    }
}

pub fn write<'a>(file_stem: &'a str, article: &'a Article, category: Option<&'a Category>)
    -> Result<(), Error> {
    copy_images(&article.images)?;
    let date = article.created_at.as_ref().unwrap_or(&NaiveDate::MIN);
    let target = parent().join(&workspace().publish.self_dir)
        .join(to_relative(&page_url(file_stem, date, category)).as_ref());
    match category {
        Some(c) =>
            Template::Article.render_write(PageTpl::from(&article, c), &target)?,
        _ => Template::About.render_write(PageTpl::single(&article), &target)?,
    }
    Ok(())
}

fn copy_images(images: &[String]) -> Result<(), Error> {
    let files = images.iter().map(|v| {
        let name = to_relative(v);
        let src = parent().join(&workspace().assets.images).join(name.as_ref());
        let target = parent().join(&workspace().publish.static_.images).join(name.as_ref());
        (src, target)
    });

    for (src, target) in files {
        fs::create_dir_all(target.parent().unwrap());
        fs::copy(&src, &target)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let text = "";
        let a = text.split("/");
        for (index, t) in text.split("/").enumerate() {
            println!("{}: {}", index, t);
        }
    }
}