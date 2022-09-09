use std::env::current_dir;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

/// 各内容目录
#[derive(Deserialize, Serialize)]
pub struct Workspace {
    // 内容目录, 默认是执行命令的当前工作目录.
    // #[serde(default = "workspace")]
    #[serde(skip_deserializing)]
    pub root: PathBuf,
    pub posts: PathBuf,
    pub notes: PathBuf,
    pub temp: PathBuf,
    pub publish: PathBuf,
    #[serde(rename = "static")]
    pub static_: PathBuf,
    pub templates: PathBuf,
    pub themes: PathBuf,
}

fn workspace() -> PathBuf {
    let current = current_dir().expect("get current directory error.");
    println!("build current dir");
    if cfg!(debug_assertions) {
        println!("debug mode");
        return current.join("example");
    }
    current
}

// contents/posts, contents/notes
// contents/posts/static, contents/notes/static
// contents/posts/<category>/static, contents/notes/<category>/static

impl Default for Workspace {
    fn default() -> Self {
        let workspace = current_dir().map(|workspace| {
            if cfg!(debug_assertions) {
                return workspace.join("example");
            }
            workspace
        }).expect("get current directory error.");

        Self {
            root: workspace.clone(),
            posts: workspace.join("posts"),
            notes: workspace.join("notes"),
            temp: workspace.join("temp"),
            publish: workspace.join("publish"),
            static_: workspace.join("static"),
            templates: workspace.join("templates"),
            themes: workspace.join("themes"),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::config::workspace::Workspace;

    #[test]
    fn test() {
        let w = Workspace::default();
        println!("{:?}", w.root.as_os_str());
    }
}