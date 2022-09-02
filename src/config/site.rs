use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Site {
    pub title: String,
    pub subtitle: Option<String>,
    pub copyright: Option<String>,
    pub footnote: Option<String>,
    // pub theme: String,
    pub author: String,
    pub lang: String,
    pub slug: String,
}
