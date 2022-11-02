use std::path::PathBuf;

use serde::Deserializer;
use serde_derive::Deserialize;

use crate::PARENT;

#[derive(Debug, Deserialize)]
pub struct Workspace {
    #[serde(deserialize_with = "de_path")]
    pub notes: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub posts: PathBuf,

    pub assets: Assets,
    pub publish: Publish,
    pub theme: Theme,

    #[serde(deserialize_with = "de_path")]
    pub temp: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    #[serde(deserialize_with = "de_path")]
    pub self_dir: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub templates: PathBuf,
    #[serde(rename = "static")]
    pub static_: Static,
}

#[derive(Debug, Deserialize)]
pub struct Publish {
    pub self_dir: PathBuf,
    #[serde(rename = "static")]
    pub static_: Static,
}

#[derive(Debug, Deserialize)]
pub struct Assets {
    #[serde(deserialize_with = "de_path")]
    pub self_dir: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub images: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Static {
    #[serde(deserialize_with = "de_path")]
    pub self_dir: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub js: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub css: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub fonts: PathBuf,
    #[serde(deserialize_with = "de_path")]
    pub images: PathBuf,
}

fn de_path<'de, D>(d: D) -> Result<PathBuf, D::Error> where D: Deserializer<'de> {
    use serde::Deserialize;
    Ok(PARENT.join(String::deserialize(d)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ws: Workspace = serde_yaml::from_str(include_str!("../workspace.yaml")).unwrap();
        println!("{:?}", ws);
    }
}
