use std::env::current_dir;
use std::path::PathBuf;

use serde_derive::Deserialize;

/// 各内容目录
#[derive(Deserialize)]
pub struct Dir {
    // 内容的根目录, 默认是执行命令的当前工作目录.
    // #[serde(default = "default_root")]
    #[serde(skip_deserializing)]
    pub root: PathBuf,
    pub posts: PathBuf,
    pub notes: PathBuf,
    // pub temp: PathBuf,
    pub dist: PathBuf,
    #[serde(rename = "static")]
    pub static_: PathBuf,
    pub templates: PathBuf,
    pub themes: PathBuf,
}

impl Default for Dir {
    fn default() -> Self {
        let root = current_dir().expect("get current dir error");
        let path = root.as_path();
        Self {
            root: root.clone(),
            posts: path.join("posts"),
            notes: path.join("notes"),
            // temp: path.join("_temp"),
            dist: path.join("dist"),
            static_: path.join("static"),
            templates: path.join("templates"),
            themes: path.join("themes"),
        }
    }
}