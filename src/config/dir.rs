use std::path::{Path, PathBuf};

use serde_derive::Deserialize;

/// 各内容目录
#[derive(Deserialize, Default)]
pub struct Dir {
    pub web: PathBuf,
    pub notes: PathBuf,
    #[serde(rename = "static")]
    pub static_: PathBuf,
    pub templates: PathBuf,
    pub themes: PathBuf,
}

impl Dir {
    pub fn from(root: &Path) -> Self {
        Self {
            web: root.join("web"),
            notes: root.join("notes"),
            static_: root.join("static"),
            templates: root.join("templates"),
            themes: root.join("themes"),
        }
    }
}