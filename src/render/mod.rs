mod render_pages;
mod render_items;

use std::borrow::Cow;
use std::path::PathBuf;
use anyhow::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::config::CONFIG;
use crate::pages::Pages;
use crate::render::render_items::render_items;
use crate::render::render_pages::render_pages;

pub fn render() -> Result<(), Error> {
    let pages = Pages::create()?;
    render_pages(&pages)?;
    render_items(&pages)?;



    Ok(())
}

pub fn render_one(k: &str) -> Result<(), Error> {
    //
    Ok(())
}

pub fn category_target(url_name: &str) -> PathBuf {
    let url_path = category_url_path(url_name);
    CONFIG.workspace.publish.join(remove_absolute(&url_path).as_ref())
}

pub fn category_url_path(url_name: &str) -> String {
    format!("/categories/{}.html", url_name)
}

pub fn page_target(file_name: &str, is_single: bool) -> PathBuf {
    let path = page_url_path(file_name, is_single);
    let path = remove_absolute(&path);
    CONFIG.workspace.publish.join(path.as_ref())
}

pub fn page_url_path(file_name: &str, is_single: bool) -> String {
    let name = REG.replace(file_name, ".html");
    match is_single {
        true => format!("/{}", name),
        _ => match &CONFIG.site.slug {
            Some(v) => format!("/{}/{}", v, name),
            _ => format!("/{}", name),
        }
    }
}

pub fn remove_absolute(txt: &str) -> Cow<str> {
    REG_ABSOLUTE.replace(txt, "")
}

pub fn resolve_image_path(path: &str) -> String {
    format!("/static/images/{}", remove_absolute(path))
}

static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(adoc)$").unwrap());
static REG_ABSOLUTE: Lazy<Regex> = Lazy::new(|| Regex::new("^/").unwrap());
