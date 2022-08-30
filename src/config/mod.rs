use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Error;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;
use crate::config::content::Content;

use crate::config::dir::Dir;
use crate::config::site::Site;

pub mod dir;
pub mod site;
pub mod content;

static CFG_TEXT: &str = include_str!("config.toml");

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut config = deserialize_config().unwrap();

    config
});

fn deserialize_config() -> Result<Config, Error> {
    let mut config: Config = toml::from_str(CFG_TEXT)?;
    config.cfg = CFG_TEXT.to_string();
    config.dir = Dir::default();
    Ok(config)
}

#[derive(Deserialize)]
pub struct Config {
    pub site: Site,
    pub content: Content,
    #[serde(skip_deserializing)]
    pub cfg: String,
    #[serde(skip_deserializing)]
    pub dir: Dir,
}

impl Config {
    pub fn dist_dir(&self) -> &Path {
        &self.dir.dist
    }

    pub fn temp_dir(&self) -> &Path {
        &self.dir.temp
    }
}
