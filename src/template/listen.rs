use std::fs;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use anyhow::Error;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use crate::CONFIG;

pub fn listen_theme() -> Result<RecommendedWatcher, Error> {
    let mut watcher = RecommendedWatcher::new(|e: Result<Event, notify::Error>| {
        on_changed(e.unwrap()).unwrap();
    }, notify::Config::default())?;
    watcher.watch(&CONFIG.workspace.posts, RecursiveMode::Recursive);
    Ok(watcher)
}

pub fn on_changed(e: Event) -> Result<(), Error> {
    let path = e.paths.first().unwrap();
    let parent = path.parent().unwrap();
    if parent == &CONFIG.workspace.theme.js ||
        parent == &CONFIG.workspace.theme.css ||
        parent == &CONFIG.workspace.theme.fonts {
        copy_files()?;
    }

    Ok(())
}

fn parse(path: &Path) -> Option<PathBuf> {
    if let Some(Some(v)) = path.extension().map(|v| v.to_str()) {
        let parent = path.parent().unwrap().to_str().unwrap();
        let publish = CONFIG.workspace.publish.join("static");
        if path.is_file() && (v == "js" || v == "css" || parent == "fonts") {
            let p = CONFIG.workspace.publish
                .join("static")
                .join(parent)
                .join(path.file_name().unwrap());
            return Some(p);
        }
    }

    None
}

pub fn copy_files() -> Result<(), Error> {
    let css_dir = CONFIG.workspace.publish.join("static/css");
    let js_dir = CONFIG.workspace.publish.join("static/js");
    let fonts_dir = CONFIG.workspace.publish.join("static/fonts");

    fs::create_dir_all(&css_dir)?;
    fs::create_dir_all(&js_dir)?;
    fs::create_dir_all(&fonts_dir)?;

    for f in load_files(&CONFIG.workspace.theme.css)? {
        fs::copy(&f, css_dir.join(f.file_name().unwrap()))?;
    }
    for f in load_files(&CONFIG.workspace.theme.js)? {
        fs::copy(&f, js_dir.join(f.file_name().unwrap()))?;
    }
    for f in load_files(&CONFIG.workspace.theme.fonts)? {
        fs::copy(&f, fonts_dir.join(f.file_name().unwrap()))?;
    }

    Ok(())
}

fn load_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let files = path.read_dir()?.into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|f| f.is_file() && f.extension().is_some())
        .collect::<Vec<PathBuf>>();
    Ok(files)
}