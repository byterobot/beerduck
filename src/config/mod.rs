use std::fs;

use anyhow::Error;
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};

use crate::config::content::Content;
use crate::config::workspace::Workspace;
use crate::config::site::Site;

pub mod workspace;
pub mod site;
pub mod content;

pub static CONFIG: Lazy<Config> = Lazy::new(|| deserialize_config().unwrap());

fn deserialize_config() -> Result<Config, Error> {
    let workspace = Workspace::default();
    let txt = fs::read_to_string(workspace.root.join("config.toml"))?;
    let mut config: Config = toml::from_str(&txt)?;
    config.workspace = workspace;
    Ok(config)
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub site: Site,
    pub content: Content,
    #[serde(skip_deserializing)]
    pub workspace: Workspace,
}
