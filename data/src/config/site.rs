use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Site {
    pub title: String,
    pub subtitle: String,
    pub copyright: String,
    pub footnote: String,
    pub author: String,
    pub lang: String,
    pub slug: String,
    pub about: String,
}
