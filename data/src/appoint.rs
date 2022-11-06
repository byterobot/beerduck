use std::path::PathBuf;

pub enum Endpoint {
    // include single path
    Page(Option<String>, String),
    // category folder name
    Category(String), // rebuild files inside category

    Categories, // scan all folders name and category.toml inside it

    Index, // rebuild all files inside all categories

    Unknown(String), //
}

impl Endpoint {
    pub fn from(path: &str) -> Option<Self> {

        todo!()
    }
}