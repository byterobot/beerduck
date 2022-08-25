use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Error;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

use crate::config::dir::Dir;
use crate::config::site::Site;

pub mod dir;
pub mod site;

static CFG_TEXT: &str = include_str!("config.toml");

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut config = deserialize_config().unwrap();
    // config.dir = Dir::from(&config.site.root);

    config
});

fn deserialize_config() -> Result<Config, Error> {
    let root = current_dir()?;
    let dir = Dir::from(&root);

    let mut config: Config = toml::from_str(CFG_TEXT)?;
    config.cfg = CFG_TEXT.to_string();
    config.root = root;
    config.dir = dir;

    Ok(config)
}

#[derive(Deserialize)]
pub struct Config {

    pub site: Site,

    #[serde(skip_deserializing)]
    pub cfg: String,

    // 内容的根目录, 默认是执行命令的当前工作目录.
    // #[serde(default = "default_root")]
    #[serde(skip_deserializing)]
    pub root: PathBuf,

    #[serde(skip_deserializing)]
    pub dir: Dir,
}
