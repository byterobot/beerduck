use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Content {
    pub single_page: Option<String>,

}