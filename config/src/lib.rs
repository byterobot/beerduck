use std::{env, fs};
use std::borrow::Cow;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;
use regex::Regex;

pub use crate::site::*;
pub use crate::workspace::*;

mod workspace;
mod site;

static LIVE_MODE: AtomicBool = AtomicBool::new(true);

pub fn set_mode(live_mode: bool) {
    LIVE_MODE.store(live_mode, Ordering::Relaxed);
}

pub fn live_mode() -> bool {
    LIVE_MODE.load(Ordering::Relaxed)
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
    if cfg!(debug_assertions) {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = dir.join("dev_location");
        if file.exists() {
            PathBuf::from(fs::read_to_string(&file).unwrap())
        } else {
            dir.parent().unwrap().join("example")
        }
    } else {
        current_dir().unwrap()
    }
});

pub fn to_relative(path: &str) -> Cow<str> {
    REG_ABSOLUTE.replace(path, "")
}

// static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(adoc)$").unwrap());
static REG_ABSOLUTE: Lazy<Regex> = Lazy::new(|| Regex::new("^/").unwrap());
