use std::path::Path;

use anyhow::Error;

use crate::pages::Pages;
use crate::render::reload::PathKind::{self, *};
use crate::render::render_items::render_items;

pub fn on_remove_render(pages: &mut Pages, path: &Path) -> Result<(), Error> {
    match PathKind::parse(path) {
        Some(Single(name)) => {
            pages.pages.remove(&name);
        },
        Some(Adoc(name)) => {
            pages.pages.remove(&name);
            pages.reindex()?;
            render_items(pages)?;
        }
        Some(Toml(_)) => {
            pages.reindex()?;
            render_items(pages)?;
        },
        Some(Folder(name)) => {
            if let Some(c) = pages.categories.remove(&name) {
                for n in c.files {
                    pages.pages.remove(&n);
                }
            }
            pages.reindex()?;
            render_items(pages)?;

        },
        _ => {}
    }
    Ok(())
}