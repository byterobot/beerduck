use std::fs;
use std::path::{Path, PathBuf};

use serde_derive::Deserialize;

#[derive(Clone)]
pub struct Category {
    pub id: u32,
    pub path: PathBuf,
    pub topic: bool,
    pub name: Option<String>, // showing name
}

#[derive(Deserialize, Default)]
struct Config {
    topic: bool,
    name: Option<String>,
}

impl Category {

    pub fn from(path: &Path) -> Self {
        let mut category = Self {
            id: 0,
            path: path.to_path_buf(),
            topic: false,
            name: None
        };
        category.update_config();
        category
    }

    pub fn update_config(&mut self) {
        let path = self.path.join("category.toml");
        let config = if path.exists() {
            match fs::read_to_string(&path) {
                Ok(text) => match toml::from_str::<Config>(&text) {
                    Ok(config) => config,
                    Err(e) => panic!("deserialize `{:?}` failed, {}", &path, e),
                },
                Err(e) => panic!("read file: `{:?}` failed, {}", &path, e),
            }
        } else {
            Config::default()
        };

        self.topic = config.topic;
        self.name = config.name;
    }

    // use for url path
    pub fn url_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    // use for url alert
    pub fn show_name(&self) -> &str {
        self.name.as_ref().map(|v| v.as_str()).unwrap_or(self.url_name())
    }

}