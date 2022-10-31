use std::path::{Path, PathBuf};

use crate::page::article::Article;
use crate::page::category::Category;

pub mod category;
pub mod article;

pub struct Page {
    pub id: u32,
    pub file: PathBuf,
    pub article: Article,
    pub category: Option<Category>,
}

impl Page {
    pub fn from(file: &Path, category: Option<Category>) -> Self {
        Self {
            id: 0,
            file: file.to_path_buf(),
            article: Article::from(file),
            category
        }
    }

    pub fn url_name(&self) -> String {
        let name = self.file.file_stem().unwrap().to_str().unwrap();
        format!("{}.html", name)
    }

    pub fn update_file(&mut self, file: &Path) {
        self.file = file.to_path_buf();
    }

    pub fn update_category(&mut self, category: Option<Category>) {
        self.category = category;
    }

    pub fn update_article(&mut self) {
        self.article = Article::from(&self.file);
    }

}


