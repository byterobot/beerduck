use std::path::{Path, PathBuf};

use anyhow::Error;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

use data::config::{site, workspace};
use data::page::category::Category;
use data::page::Page;

pub struct DocMap {
    pub docs: Vec<(Category, Vec<Page>)>,
    pub about: Page,
}
impl DocMap {
    pub fn create() -> Self {
        Self {
            docs: read_docs().expect("read adoc files failed"),
            about: Page::from(&workspace().posts.join(&site().about)),
        }
    }

    pub fn listen_change() -> Result<RecommendedWatcher, Error> {
        let mut watcher = RecommendedWatcher::new(|e: Result<Event, notify::Error>| {
            if let Ok(event) = e {

            }
        }, notify::Config::default())?;
        watcher.watch(&workspace().posts, RecursiveMode::Recursive)?;
        watcher.watch(&workspace().theme.templates, RecursiveMode::Recursive)?;
        watcher.watch(&workspace().theme.css, RecursiveMode::Recursive)?;
        watcher.watch(&workspace().theme.js, RecursiveMode::Recursive)?;

        Ok(watcher)
    }
}

pub fn read_docs() -> Result<Vec<(Category, Vec<Page>)>, Error> {
    let dirs = workspace().posts.read_dir()?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect::<Vec<PathBuf>>();

    let mut vec = vec![];
    for path in dirs {
        vec.push((Category::from(&path), load_files(&path)?));
    }
    Ok(vec)
}

fn load_files(path: &Path) -> Result<Vec<Page>, Error> {
    let mut pages = vec![];
    for p in path.read_dir()? {
        let file = p?.path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.ends_with(".adoc") && name != "index.adoc" {
            pages.push(Page::from(&file));
        }
    }
    Ok(pages)
}
