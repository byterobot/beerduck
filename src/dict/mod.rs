use std::collections::HashMap;

use anyhow::Error;
use once_cell::sync::Lazy;

use crate::config::CONFIG;
use crate::dict::adoc::AdocFile;
use crate::dict::category::Category;

pub(crate) mod adoc;
pub(crate) mod category;

pub static DICT: Lazy<Dict> = Lazy::new(|| Dict::create().expect("create dict error"));

pub struct Dict {
    adoc_map: HashMap<String, AdocFile>, // name.adoc -> AdocFile
    adoc_category_map: HashMap<String, String>, // name.adoc -> category name
    category_map: HashMap<String, Category>, // category name -> Category
}

impl Dict {
    fn create() -> Result<Self, Error> {
        let config = &CONFIG;
        let adoc_map = adoc::build_adoc_map(&config.workspace)?;
        let category_map = category::build_category_map(config)?;
        let mut adoc_category_map = HashMap::new();
        for (k, v) in &category_map {
            for a in &v.adoc_files {
                adoc_category_map.insert(a.clone(), k.clone());
            }
        }
        let dict = Self { adoc_map, adoc_category_map, category_map };
        Ok(dict)
    }

    pub fn adoc_map(&self) -> &HashMap<String, AdocFile> {
        &self.adoc_map
    }

    pub fn get_adoc(&self, adoc_name: &str) -> Option<&AdocFile> {
        self.adoc_map.get(adoc_name)
    }

    pub fn adoc_category_map(&self) -> &HashMap<String, String> {
        &self.adoc_category_map
    }

    pub fn get_category_name(&self, adoc_name: &str) -> Option<&String> {
        self.adoc_category_map.get(adoc_name)
    }

    pub fn category_map(&self) -> &HashMap<String, Category> {
        &self.category_map
    }

    pub fn get_category(&self, category_name: &str) -> Option<&Category> {
        self.category_map.get(category_name)
    }

}

#[cfg(test)]
mod test {
    use crate::dict::DICT;

    #[test]
    fn test() {
        let a = &DICT.adoc_map;
        // for (k, v) in a {
        //     println!("{} -> {:?}", k, v);
        // }

        let b = &DICT.category_map;
        // for (k, v) in b {
        //     println!("{} -> {:?}", k, v);
        // }

        let c = &DICT.adoc_category_map;
        for (k, v) in c {
            println!("{} -> {}", k, v);
        }

        println!("test");
    }
}