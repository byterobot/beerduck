use std::path::PathBuf;

use anyhow::Error;
use serde::{Deserialize, Deserializer};

use crate::config::ROOT;

#[derive(Deserialize)]
pub struct Workspace {
    #[serde(deserialize_with = "de_path")]
    pub assets: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub notes: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub posts: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub render: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub publish: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub temp: PathBuf,
    pub theme: Theme,
}

#[derive(Deserialize)]
pub struct Theme {
    #[serde(deserialize_with = "de_path")]
    pub templates: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub js: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub css: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub fonts: PathBuf,
}

fn de_path<'de, D>(d: D) -> Result<PathBuf, D::Error> where D: Deserializer<'de> {
    Ok(ROOT.join(String::deserialize(d)?))
}
