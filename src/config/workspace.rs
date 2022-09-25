use std::env::current_dir;
use std::path::{Path, PathBuf};

use serde_derive::Deserialize;

/// 各内容目录
#[derive(Deserialize)]
pub struct Workspace {
    // 内容目录, 默认是执行命令的当前工作目录.
    // #[serde(default = "workspace")]
    #[serde(skip_deserializing)]
    pub root: PathBuf,
    pub posts: PathBuf,
    pub notes: PathBuf,
    pub temp: PathBuf,
    pub publish: PathBuf,
    pub assets: PathBuf,
    pub theme: Theme,
}

impl Default for Workspace {
    fn default() -> Self {
        let workspace = workspace();
        Self {
            root: workspace.clone(),
            posts: workspace.join("posts"),
            notes: workspace.join("notes"),
            temp: workspace.join("temp"),
            publish: workspace.join("publish"),
            assets: workspace.join("assets"),
            theme: Theme::new(&workspace),
        }
    }
}

#[derive(Deserialize)]
pub struct Theme {
    pub templates: PathBuf,
    pub js: PathBuf,
    pub css: PathBuf,
    pub fonts: PathBuf,
}

impl Theme {
    fn new(workspace: &Path) -> Self {
        Self {
            templates: workspace.join("theme/templates"),
            js: workspace.join("theme/js"),
            css: workspace.join("theme/css"),
            fonts: workspace.join("theme/fonts"),
        }
    }
}

fn workspace() -> PathBuf {
    match cfg!(debug_assertions) {
        true => toml::from_str::<Dev>(include_str!("../../dev.toml")).unwrap().root,
        _ => current_dir().unwrap(),
    }
}

#[derive(Deserialize)]
struct Dev {
    root: PathBuf,
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