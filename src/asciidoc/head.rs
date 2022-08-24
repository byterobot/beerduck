use chrono::{Date, DateTime, Utc};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Head {
    #[serde(default = "Utc::now")]
    date: DateTime<Utc>, // 发布时间
    #[serde(default)]
    category_path: bool, // category 目录是否作为 path 的一部分, 默认 false.
    #[serde(default)]
    outline: bool, // 是否显式 outline, 默认是 true
    #[serde(default = "Vec::new")]
    tags: Vec<String>, // ["tag1", "tag2"]
}

impl Head {
    pub fn new() -> Self {
        Self::from("{}")
    }

    pub fn from(text: &str) -> Self {
        toml::from_str(text).expect("Can't parse head text.")
    }
}

#[cfg(test)]
mod test {
    use serde_derive::Deserialize;

    #[derive(Deserialize)]
    struct C {
        abc: u8
    }

    #[test]
    fn test() {
        let t = "abc= 1";
        let c = toml::from_str::<C>(t).unwrap();
    }
}