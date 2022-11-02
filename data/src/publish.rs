use std::path::{Path, PathBuf};

use anyhow::Error;

use config::{site, workspace};

use crate::page::{Category, Article};
use crate::page::Page;

pub struct Pages {
    pub pages: Vec<(Category, Vec<Page>)>,
    pub about: Page,
}

impl Pages {
    pub fn create() -> Self {
        Self {
            pages: read_docs().expect("read adoc files failed"),
            about: Page::from(&workspace().posts.join(&site().about)),
        }
    }

}

pub fn read_docs() -> Result<Vec<(Category, Vec<Page>)>, Error> {
    let dirs = workspace().posts.read_dir()?
        .into_iter()
        .filter_map(|r| {
            if let Ok(v) = r {
                let slug_path = PathBuf::from(&site().slug);
                let mut slug = slug_path.components()
                    .filter(|c| c.as_os_str() != "/")
                    .filter_map(|c| c.as_os_str().to_str());
                let path = v.path();
                if path.is_dir() &&
                    !path.ends_with("static") &&
                    !path.ends_with(slug.next().unwrap_or_default()) {
                    return Some(path);
                }
            }
            None
        }).collect::<Vec<PathBuf>>();

    let mut vec = vec![];
    for path in dirs {
        vec.push((Category::from(&path), load_files(&path)?));
    }
    Ok(vec)
}

fn load_files(path: &Path) -> Result<Vec<Page>, Error> {
    let mut pages = vec![];
    for p in path.read_dir()? {
        let file = p?.path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.ends_with(".adoc") && name != "index.adoc" {
            pages.push(Page::from(&file));
        }
    }
    Ok(pages)
}
