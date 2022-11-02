use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde_derive::Deserialize;

pub use crate::site::*;
pub use crate::workspace::*;

mod workspace;
mod site;

pub fn dev_mode() -> bool {
    // todo
    true
}

pub fn site() -> &'static Site {
    &SITE
}

pub fn workspace() -> &'static Workspace {
    &WORKSPACE
}

static SITE: Lazy<Site> = Lazy::new(|| {
    let file = PARENT.join("config.toml");
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
