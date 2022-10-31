use std::ops::Deref;
use std::path::{Path, PathBuf};

use anyhow::Error;
use log::debug;
use notify::{Event, EventKind};
use notify::event::ModifyKind;

use crate::config::CONFIG;
use crate::pages::page::Page;
use crate::pages::Pages;

mod create;
mod modify;
mod remove;
mod modify_data;

pub fn listen_changed(pages: &mut Pages, event: Event) -> Result<(), Error> {
    let path = event.paths.first().unwrap();
    debug!("path -> {:?}", path);
    let kind = PathKind::parse(path);
    debug!("kind -> {:?}", kind);
    match event.kind {
        EventKind::Create(_) => create::on_create_render(pages, path)?,
        EventKind::Modify(k) => match k {
            ModifyKind::Name(_) => modify::on_modify_render(pages, path)?,
            ModifyKind::Data(_) => modify_data::on_modify_data_render(pages, path)?,
            _ => {}
        }
        EventKind::Remove(_) => remove::on_remove_render(pages, path)?,
        _ => {}
    }

    Ok(())
}

// about.adoc, category name, adoc inside category.
fn is_valid(path: &Path) -> bool {
    let root = &CONFIG.workspace.posts;
    (path.is_file() && path == root.join("about.doc")) ||
        (path.is_dir() && path == root.join(path.file_name().unwrap())) ||
        (path.is_file() && path ==
            root.join(path.parent().unwrap().file_name().unwrap())
                .join(path.file_name().unwrap())
        )
}

pub fn file_name(path: &Path) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}

#[derive(Debug)]
enum PathKind {
    Single(String), Adoc(String), Toml(String), Folder(String),
}

impl PathKind {
    fn parse(path: &Path) -> Option<Self> {
        let root = &CONFIG.workspace.posts;
        let name = file_name(path)?;
        if path.is_file() {
            if path == root.join("about.doc") {
                return Some(Self::Single(name));
            } else if path == root.join(file_name(path.parent()?)?).join(&name) {
                // let file_name = path.file_name()?.to_str()?;
                if name.as_str() == "category.toml" {
                    return Some(Self::Toml(name));
                } else if name.ends_with(".adoc") {
                    return Some(Self::Adoc(name));
                }
            }
        } else if path.is_dir() && path == root.join(&name) {
            return Some(Self::Folder(name));
        }

        None
    }
}
