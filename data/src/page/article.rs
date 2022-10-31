use std::path::Path;

use chrono::NaiveDate;

use crate::asciidoc;

// asciidoc file contents
pub struct Article {
    pub title: Option<String>,
    pub author: Option<String>,
    pub lang: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub created_at: Option<NaiveDate>,
    pub updated_at: Option<NaiveDate>,
    pub toc_html: Option<String>, // id "toc>ol"
    pub content_html: Option<String>, // id "content"
    pub images: Vec<String>,
}

impl Article {
    pub fn from(file: &Path) -> Self {
        asciidoc::parse_doc(file).unwrap()
    }
}