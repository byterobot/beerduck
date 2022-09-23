use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;

#[derive(Deserialize, Serialize)]
pub struct Site {
    pub title: String,
    pub subtitle: Option<String>,
    pub copyright: Option<String>,
    pub footnote: Option<String>,
    // pub theme: String,

    #[serde(skip_serializing)]
    pub author: String,
    pub lang: String,
    #[serde(deserialize_with = "de_slug")]
    pub slug: Option<String>,
}

// 去掉第一个斜线/
fn de_slug<'de, D>(d: D) -> Result<Option<String>, D::Error> where D: Deserializer<'de> {
    let v = Option::<String>::deserialize(d)?
        .map(|v| v.replacen(r"^\/", "", 1));
    Ok(v)
}

