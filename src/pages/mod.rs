use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Error};

use crate::config::CONFIG;
use crate::pages::category::Category;
use crate::pages::page::Page;

pub mod page;
pub mod category;
pub mod asciidoc;

pub struct Pages {
    pub pages: HashMap<String, Page>,
    pub categories: HashMap<String, Category>,
    pub categories_name: HashMap<String, String>,
}

impl Pages {
    pub fn create() -> Result<Self, Error> {
        let mut pages = HashMap::new();
        for p in load_adoc_files(&CONFIG.workspace.posts)? {
            let name = p.file_name().unwrap().to_str().unwrap().to_string();
            pages.insert(name, Page::from(&p)?);
        }
        let mut p = Self {pages, categories: Default::default(), categories_name: Default::default()};
        p.reindex()?;

        Ok(p)
    }

    pub fn reindex(&mut self) -> Result<(), Error> {
        self.categories = load_categories(&CONFIG.workspace.posts)?;
        let mut names = HashMap::new();
        self.categories.iter().for_each(|(k, v)| {
            v.files.iter().for_each(|v| { names.insert(v.clone(), k.clone()); });
        });
        self.categories_name = names;
        Ok(())
    }
}


fn load_adoc_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut adoc_files = Vec::from([path.join("about.adoc")]);
    let dirs = path.read_dir()?.into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect::<Vec<PathBuf>>();

    for p in dirs {
        let vec = p.read_dir()?.into_iter()
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap().path())
            .filter(|f| is_adoc(f))
            .collect::<Vec<PathBuf>>();
        adoc_files.extend(vec);
    }
    Ok(adoc_files)
}

fn is_adoc(path: &Path) -> bool {
    if path.is_file() {
        if let Some(n) = path.file_name().unwrap().to_str() {
            return n.ends_with(".adoc") && n != "index.adoc";
        }
    }
    false
}

fn load_categories(path: &Path) -> Result<HashMap<String, Category>, Error> {
    let mut map = HashMap::new();
    for a in load_adoc_files(path)? {
        let file_name = a.file_name().unwrap().to_str().unwrap();
        if file_name == "about.adoc" {
            continue;
        }

        let category_name = a.parent().unwrap().file_name().unwrap().to_str().unwrap();
        if !map.contains_key(category_name) {
            map.insert(category_name.to_string(), load_category(a.parent().unwrap())?);
        }

        if let Some(c) = map.get_mut(category_name) {
            c.files.push(file_name.into());
        }
    }
    Ok(map)
}

pub fn load_category(path: &Path) -> Result<Category, Error> {
    let category_name = path.file_name().unwrap().to_str().unwrap();
    let config = path.join("category.toml");
    let mut c = match config.exists() {
        true => toml::from_str(&fs::read_to_string(config)?)?,
        _ => Category::default(),
    };
    c.name = category_name.into();

    Ok(c)
}


#[cfg(test)]
mod test {
    use crate::config::CONFIG;
    use crate::pages::{load_adoc_files, load_categories, Pages};

    #[test]
    fn test_() {
        let pages = Pages::create().unwrap();
        for (k, v) in pages.categories_name {
            println!("{} -> {}", k, v);
        }
    }

    #[test]
    fn test() {
        let files = load_adoc_files(&CONFIG.workspace.posts).unwrap();
        println!("{:?}", files);

        let categories = load_categories(&CONFIG.workspace.posts).unwrap();
        for (name, c) in categories {
            println!("{}", name);
            println!("{:?}", c);
        }
    }
}

