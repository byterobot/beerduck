use std::path::Path;

use anyhow::Error;
use crate::pages::page::Page;

use crate::pages::Pages;
use crate::render::listen::PathKind::{self, *};
use crate::render::render_items::render_items;
use crate::render::render_pages::render_page;

pub fn on_modify_data_render(pages: &mut Pages, path: &Path) -> Result<(), Error> {
    match PathKind::parse(path) {
        Some(Single(name)) => render_page(pages, &name)?,
        Some(Adoc(name)) => {
            pages.pages.insert(name.clone(), Page::from(path)?);
            render_page(pages, &name)?;
            pages.reindex()?;
            render_items(pages)?;
        }
        Some(Toml(_)) => {
            pages.reindex()?;
            render_items(pages)?;
        },
        Some(Folder(_)) => {
            pages.reindex()?;
            render_items(pages)?;
        },
        _ => {}
    }
    Ok(())
}
