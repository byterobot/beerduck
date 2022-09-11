use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Error;
use regex::Regex;

use crate::config::CONFIG;
use crate::config::workspace::Workspace;

#[derive(Debug)]
pub struct AdocFile {
    pub file_name: String,
    pub category_name: Option<String>, // None is single page
    pub adoc_absolute: PathBuf,
    pub html_relative: PathBuf,
    pub html_absolute: PathBuf,
}

pub fn build_adoc_map(workspace: &Workspace) -> Result<HashMap<String, AdocFile>, Error> {
    // let workspace = &CONFIG.workspace;
    let mut map = HashMap::new();
    for dir in workspace.posts.read_dir()? {
        let path = dir?.path();
        if path.is_file() { // 根目录的文件
            if let Some(v) = read_adoc(path.as_path(), None)? {
                map.insert(v.file_name.clone(), v);
            }
        } else if path.is_dir() { // category 目录
            let name = path.file_name().unwrap()
                .to_str().unwrap().to_string(); // 必然存在, 文件名或文件夹名
            for d in path.read_dir()? {
                if let Some(v) = read_adoc(&d?.path(), Some(&name))? {
                    map.insert(v.file_name.clone(), v);
                }
            }
        }
    }
    Ok(map)
}

fn read_adoc(path: &Path, category: Option<&String>) -> Result<Option<AdocFile>, Error> {
    let name = match path.file_name() {
        Some(v) => v,
        _ => return Ok(None),
    };

    if let Some(name) = name.to_str() {
        let reg = Regex::new(r"\.(adoc)$").unwrap();
        let html_name = reg.replace(name, ".html").to_string();
        // let html_name = name.replace(r"\.(adoc)$", ".html");
        let html_rel = match category.is_some() && CONFIG.site.slug.is_some() {
            true => format!("{}/{}", CONFIG.site.slug.as_ref().unwrap(), html_name),
            _ => format!("{}", html_name), // single page
        };

        if name.ends_with(".adoc") {
            let file = AdocFile {
                file_name: name.to_string(),
                category_name: category.map(|v| v.clone()),
                adoc_absolute: path.to_path_buf(),
                html_relative: PathBuf::from(&html_rel),
                html_absolute: CONFIG.workspace.publish.join(html_rel),
            };
            return Ok(Some(file))
        }
    }
    Ok(None)
}