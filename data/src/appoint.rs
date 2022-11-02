use std::path::PathBuf;

pub enum Appoint {
    // include single path
    Page(PathBuf),
    // category folder name
    Category(String), // rebuild files inside category

    Categories, // scan all folders name and category.toml inside it

    Index, // rebuild all files inside all categories

    Unknown(String), //
}

impl Appoint {
    pub fn from(url: &str) -> Self {


        todo!()
    }
}