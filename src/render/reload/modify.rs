use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::Error;
use once_cell::sync::Lazy;

use crate::pages::page::Page;
use crate::pages::Pages;
use crate::render::reload::file_name;
use crate::render::reload::PathKind::{self, *};
use crate::render::render_items::render_items;
use crate::render::render_pages::render_page;

static CACHE: Lazy<Mutex<PageCache>> = Lazy::new(|| Default::default());

pub fn on_modify_render(pages: &mut Pages, path: &Path) -> Result<(), Error> {
    let mut cache = CACHE.lock().unwrap();
    if !cache.cached {
        *cache = PageCache { cached: true, page: None, };
        if let Some(Adoc(_)) | Some(Single(_)) = PathKind::parse(path) {
            cache.page = pages.pages.remove(&file_name(path).unwrap());
        }
        return Ok(());
    }

    let kind = PathKind::parse(path);
    if let Some(Adoc(_)) | Some(Single(_)) = &kind {
        let name = file_name(path).unwrap();
        match &cache.page {
            Some(p) => { pages.pages.insert(name, p.clone()); },
            _ => {
                pages.pages.insert(name.clone(), Page::from(path)?);
                render_page(pages, &name)?;
            }
        }
    }

    *cache = PageCache::default();

    if kind.is_some() { // 有效更改文件名
        pages.reload_index()?;
        render_items(pages)?;
    }

    Ok(())
}

#[derive(Default)]
struct PageCache {
    cached: bool,
    page: Option<Page>,
}