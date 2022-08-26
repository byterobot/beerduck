use std::path::{Path, PathBuf};

pub struct DirTree {
    pub category: Vec<PathBuf>,
    pub single_page: Option<PathBuf>,
    pub index: PathBuf,
}

impl DirTree {

    pub fn from(posts: &Path) -> Self {
        todo!()
    }
}