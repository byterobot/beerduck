use std::borrow::Cow;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;
use regex::Regex;
use serde_derive::Deserialize;

pub use crate::site::*;
pub use crate::workspace::*;

mod workspace;
mod site;

static DEV_MODE: AtomicBool = AtomicBool::new(true);

pub fn set_mode(dev_mode: bool) {
    DEV_MODE.store(dev_mode, Ordering::Relaxed);
}

pub fn dev_mode() -> bool {
    DEV_MODE.load(Ordering::Relaxed)
}

pub fn site() -> &'static Site {
    &SITE
}

pub fn workspace() -> &'static Workspace {
    &WORKSPACE
}

pub fn parent() -> &'static Path {
    &PARENT
}

static SITE: Lazy<Site> = Lazy::new(|| {
    let file = parent().join("config.toml");
    match fs::read_to_string(&file) {
        Ok(text) => toml::from_str::<Site>(&text)
            .expect("deserialize config.toml error"),
        Err(e) => panic!("read file config.toml error, {}", e),
    }
});
static WORKSPACE: Lazy<Workspace> = Lazy::new(||
    serde_yaml::from_str::<Workspace>(include_str!("../workspace.yaml"))
        .expect("deserialize workspace.yaml error")
);

pub(crate) static PARENT: Lazy<PathBuf> = Lazy::new(|| {
    #[derive(Deserialize)]
    struct Parent { parent: PathBuf, }
    match cfg!(debug_assertions) {
        true => toml::from_str::<Parent>(include_str!("../dev.toml")).unwrap().parent,
        _ => current_dir().unwrap(),
    }
});

pub fn make_relative_path(txt: &str) -> Cow<str> {
    REG_ABSOLUTE.replace(txt, "")
}

// static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(adoc)$").unwrap());
static REG_ABSOLUTE: Lazy<Regex> = Lazy::new(|| Regex::new("^/").unwrap());
