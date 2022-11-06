use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Workspace {
    // #[serde(deserialize_with = "de_path")]
    pub notes: String,
    // #[serde(deserialize_with = "de_path")]
    pub posts: String,

    pub assets: Assets,
    pub publish: Publish,
    pub theme: Theme,

    // #[serde(deserialize_with = "de_path")]
    pub temp: String,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    // #[serde(deserialize_with = "de_path")]
    pub self_dir: String,
    // #[serde(deserialize_with = "de_path")]
    pub templates: String,
    #[serde(rename = "static")]
    pub static_: Static,
}

#[derive(Debug, Deserialize)]
pub struct Publish {
    pub self_dir: String,
    #[serde(rename = "static")]
    pub static_: Static,

    pub categories_dir: String,
    pub categories_index: String,
}

#[derive(Debug, Deserialize)]
pub struct Assets {
    // #[serde(deserialize_with = "de_path")]
    pub self_dir: String,
    // #[serde(deserialize_with = "de_path")]
    pub images: String,
}

#[derive(Debug, Deserialize)]
pub struct Static {
    // #[serde(deserialize_with = "de_path")]
    pub self_dir: String,
    // #[serde(deserialize_with = "de_path")]
    pub js: String,
    // #[serde(deserialize_with = "de_path")]
    pub css: String,
    // #[serde(deserialize_with = "de_path")]
    pub fonts: String,
    // #[serde(deserialize_with = "de_path")]
    pub images: String,
}

/*fn de_path<'de, D>(d: D) -> Result<PathBuf, D::Error> where D: Deserializer<'de> {
    use serde::Deserialize;
    Ok(parent().join(String::deserialize(d)?))
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ws: Workspace = serde_yaml::from_str(include_str!("../workspace.yaml")).unwrap();
        println!("{:?}", ws);
    }
}
