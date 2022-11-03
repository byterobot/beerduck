use std::path::Path;

use anyhow::Error;
use chrono::{Datelike, NaiveDate};

use config::{parent, site, workspace};
use render::Template;

use crate::page::{Article, Category};
use crate::template::page::PageTpl;

pub fn gen<'a>(article: &'a Article, category: Option<&'a Category>) -> Result<String, Error> {
    match category {
        Some(c) => Template::Article.render(PageTpl::from(article, c)),
        _ => Template::About.render(PageTpl::single(article)),
    }
}

pub fn write<'a>(file_stem: &'a str, article: &'a Article, category: Option<&'a Category>) -> Result<(), Error> {
    let date = article.created_at.as_ref().unwrap_or(&NaiveDate::MIN);
    let target = parent().join(&workspace().publish.self_dir)
        .join(&page_url(file_stem, date, category));
    match category {
        Some(c) =>
            Template::Article.render_write(PageTpl::from(&article, c), &target)?,
        _ => Template::About.render_write(PageTpl::single(&article), &target)?,
    }
    Ok(())
}

pub fn page_url<'a>(file_stem: &str, date: &'a NaiveDate, category: Option<&Category>) -> String {
    match category {
        Some(category) => {
            let url_path = url_path(date, category);
            match url_path.is_empty() {
                true => format!("/{}.html", file_stem),
                _ => format!("/{}/{}.html", url_path, file_stem),
            }
        },
        _ => format!("/{}.html", file_stem),
    }
}

fn url_path(date: &NaiveDate, category: &Category) -> String {
    if category.topic {
        return category.name.to_string();
    }
    let mut text = String::new();
    for v in site().slug.split("/").map(|v| v.trim()).filter(|v| !v.is_empty()) {
        if !text.is_empty() { text.push('/'); }
        match v {
            "y" | "Y" => text.push_str(&format!("{:02}", date.month())),
            "m" | "M" => text.push_str(&date.year().to_string()),
            "d" | "D" => text.push_str(&format!("{:02}", date.day())),
            _ => text.push_str(v),
        }
    }
    text
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