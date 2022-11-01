use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

use anyhow::Error;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

pub use crate::config::site::Site;
pub use crate::config::workspace::{Theme, Workspace};

mod workspace;
mod site;

pub fn dev_mode() -> bool {
    // todo
    false
}

pub fn site() -> &'static Site {
    &SITE
}

pub fn workspace() -> &'static Workspace {
    &WORKSPACE
}

static SITE: Lazy<Site> = Lazy::new(|| {
    let file = ROOT.join("config.toml");
    match fs::read_to_string(&file) {
        Ok(text) => toml::from_str::<Site>(&text)
            .expect("deserialize config.toml error"),
        Err(e) => panic!("read file config.toml error, {}", e),
    }
});
static WORKSPACE: Lazy<Workspace> = Lazy::new(||
    toml::from_str::<Workspace>(include_str!("workspace.toml"))
        .expect("deserialize workspace.toml error")
);

pub static ROOT: Lazy<PathBuf> = Lazy::new(|| {
    #[derive(Deserialize)]
    struct Root { root: PathBuf, }
    match cfg!(debug_assertions) {
        true => toml::from_str::<Root>(include_str!("dev.toml")).unwrap().root,
        _ => current_dir().unwrap(),
    }
});
