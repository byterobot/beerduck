use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Category {
    #[serde(skip_deserializing)]
    pub name: String,
    #[serde(skip_deserializing)]
    pub files: Vec<String>,

    pub position: u16, // 在 category 目录中的排序
    pub date_asc: bool,
    pub pin: Option<String>, // example.adoc
    pub alias: Option<String>, // <alias_name>.html
}
