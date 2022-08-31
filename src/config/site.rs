use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Site {
    pub title: String,
    pub subtitle: Option<String>,
    pub copyright: Option<String>,
    pub footnote: Option<String>,
    pub theme: String,
    pub lang: String,
    pub path: String,
}
