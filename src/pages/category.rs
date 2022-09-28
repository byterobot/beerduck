use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Category {
    #[serde(skip_deserializing)]
    pub name: String,
    #[serde(skip_deserializing)]
    pub files: Vec<String>,

    #[serde(default)]
    pub position: u16, // 在 category 目录中的排序
    #[serde(default)]
    pub date_asc: bool,
    #[serde(default)]
    pub pin: Option<String>, // example.adoc
    #[serde(default)]
    pub alias: Option<String>, // <alias_name>.html
}
