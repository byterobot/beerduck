use std::path::Path;

use anyhow::Error;

use crate::pages::{load_category, Pages};
use crate::pages::page::Page;
use crate::render::reload::file_name;
use crate::render::reload::PathKind::{self, *};
use crate::render::render_items::render_items;
use crate::render::render_pages::render_page;

pub fn on_create_render(pages: &mut Pages, path: &Path) -> Result<(), Error> {
    let kind = PathKind::parse(path);
    if let Some(Adoc(_)) | Some(Single(_)) = &kind {
        let name = file_name(path).unwrap();
        pages.pages.insert(name.clone(), Page::from(path)?);
        render_page(pages, &name)?;
    }

    if kind.is_some() { // 有效更改文件名
        pages.reindex()?;
        render_items(pages)?;
    }

    Ok(())
}

fn parse_category(path: &Path) -> Option<String> {
    Some(path.parent()?.file_name()?.to_str()?.to_string())
}