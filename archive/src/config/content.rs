use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Content {
    pub single_page: Option<String>,

}