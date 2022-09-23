use std::path::Path;

use anyhow::Error;
use crate::pages::page::Page;
use crate::pages::{load_category, Pages};
use crate::render::reload::adoc_name;

use crate::render::reload::PathKind::{self, *};
use crate::render::render_items::{render_categories, render_category, render_index};
use crate::render::render_pages::render_page;

pub fn on_create_render(pages: &mut Pages, path: &Path) -> Result<(), Error> {
    let kind = PathKind::parse(path);
    if let Some(Adoc(_)) | Some(Single(_)) = &kind {
        let name = adoc_name(path).unwrap();
        pages.pages.insert(name.clone(), Page::from(path)?);
        render_page(pages, &name)?;
    }

    if kind.is_some() { // 有效更改文件名
        pages.rebuild_index()?;

        // 重新渲染所有列表
        render_category(&pages)?;
        render_categories(&pages)?;
        render_index(&pages)?;
    }

    Ok(())
}

fn parse_category(path: &Path) -> Option<String> {
    Some(path.parent()?.file_name()?.to_str()?.to_string())
}