use std::path::{Path, PathBuf};

use anyhow::{anyhow, Error};

/// 文章分类内容.
/// 默认情况下, 会自动生成分类的首页, 路径是 `categories/<category name>`.
/// 自定义分类首页: 分类目录下 _index.adoc 会取代自动生成, 成为新的分类首页.
/// 为分类指定新的路径: 在分类目录下创建 `_new-path.url` 文件, 此时分类的首页是 `categories/new-path`.
pub struct Category {
    pub name: String, // `category name`
    pub new_path: String, // `_new-path.url` 中的 `new-path`, 若不存在则是 `category name`

    pub index: Option<PathBuf>, // _index.adoc
    pub vec: Vec<PathBuf>,
}

impl Category {
    pub fn from(category: &Path) -> Self {
        match read_dir(category) {
            Ok(v) => v,
            Err(e) => panic!("Create category `{:?}` failed: {}", category, e),
        }
    }

    fn generate(&self, target: &Path) {

    }
}


fn read_dir(category: &Path) -> Result<Category, Error> {
    let (name, new_path) = read_name(category)?;
    let mut index = None;
    let mut vec = vec![];
    for dir in category.read_dir()? {
        let dir = dir?;
        if let Some(name) = dir.file_name().to_str() {
            if index == None && name == "_index.adoc" {
                index = Some(dir.path());
            } else if name.ends_with(".adoc") {
                vec.push(dir.path());
            }
        }
    }

    Ok(Category { name, new_path, index, vec })
}

fn read_name(category: &Path) -> Result<(String, String), Error> {
    let category_name = category
        .file_name().ok_or_else(|| anyhow!("Invalid category name"))?
        .to_str().ok_or_else(|| anyhow!("Invalid category name"))?
        .to_string();

    for dir in category.read_dir()? {
        let a = dir?;
        if let Some(name) = a.file_name().to_str() {
            if name.ends_with(".url") && name.starts_with("_") {
                let new_name = Path::new(name)
                    .file_stem().ok_or_else(|| anyhow!("Invalid new path"))?
                    .to_str().ok_or_else(|| anyhow!("Invalid new path"))?
                    .replacen('_', "", 1);
                return Ok((category_name, new_name));
            }
        }
    }

    let new_name = category_name.clone();
    Ok((category_name, new_name))
}