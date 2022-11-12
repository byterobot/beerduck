use std::fs;
use std::path::Path;

use anyhow::Error;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Category {
    pub name: String,
    pub show_name: String,
    pub topic: bool,
}

#[derive(Deserialize, Default)]
struct Config {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub topic: bool,
}

impl Category {
    pub fn from(path: &Path) -> Result<Self, Error> {
        let mut category = Category::default();
        category.name = path.file_name().unwrap().to_str().unwrap().to_string();
        category.show_name = category.name.clone();

        let config = path.join("category.toml");
        if config.exists() {
            let text = fs::read_to_string(&config)?;
            let c = toml::from_str::<Config>(&text)?;
            category.topic = c.topic;
            if let Some(name) = c.name {
                category.show_name = name;
            }
        }

        Ok(category)
    }
}
