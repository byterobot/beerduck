use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

use anyhow::Error;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::{Lazy, OnceCell};
use regex::Regex;

use crate::config::CONFIG;
use crate::pages::Pages;
use crate::render::render_items::render_items;
use crate::render::render_pages::render_pages;

mod render_pages;
mod render_items;
mod listen;

static PAGES: OnceCell<Mutex<Pages>> = OnceCell::new();

pub fn init() {
    PAGES.set(Mutex::new(Pages::create().unwrap()));
}

pub fn render() -> Result<(), Error> {
    let pages = PAGES.get().unwrap().lock().unwrap();
    render_pages(&pages)?;
    render_items(&pages)?;
    Ok(())
}

pub fn listen_posts() -> Result<RecommendedWatcher, Error> {
    let mut watcher = RecommendedWatcher::new(|e: Result<Event, notify::Error>| {
        let mut pages = PAGES.get().unwrap().lock().unwrap();
        listen::listen_changed(&mut pages, e.unwrap()).unwrap();
    }, notify::Config::default())?;
    watcher.watch(&CONFIG.workspace.posts, RecursiveMode::Recursive);
    Ok(watcher)
}

pub fn home_target() -> PathBuf {
    CONFIG.workspace.publish.join("index.html")
}

pub fn home_url_path() -> String {
    "/".into()
}

pub fn categories_target() -> PathBuf {
    CONFIG.workspace.publish.join(remove_absolute(&categories_url_path()).as_ref())
}

pub fn categories_url_path() -> String {
    "/categories.html".into()
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

#[cfg(test)]
mod test {
    use crate::render::render;

    #[test]
    fn test() {
        render().unwrap();
    }
}
