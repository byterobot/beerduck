use once_cell::sync::Lazy;
use serde_derive::Deserialize;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    toml::from_str(include_str!("../config.toml"))
        .expect("Deserialize `config.toml` error")
});

#[derive(Deserialize)]
pub struct Config {
    pub site: Site,
    pub content: Content,
}

#[derive(Deserialize)]
pub struct Site {
    pub title: String,
    pub subtitle: Option<String>,
    pub copyright: Option<String>,
    pub footnote: Option<String>,
}

#[derive(Deserialize)]
pub struct Content {
    site_dir: String,
    notes_dir: String,
    // single_pages: Vec<String>,
}