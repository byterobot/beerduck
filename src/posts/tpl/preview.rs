use serde_derive::Serialize;

use crate::config::site::Site;
use crate::posts::{Category, Generated};

#[derive(Serialize)]
pub struct IndexTpl {

}

#[derive(Serialize)]
pub struct CategoryTpl<'a> {
    site: &'a Site
}

impl<'a> CategoryTpl<'a> {
    pub fn create(g: &Generated) -> Self {
        todo!()
    }
}


#[derive(Serialize)]
pub struct CategoriesTpl<'a> {
    site: &'a Site
}

impl<'a> CategoriesTpl<'a> {
    pub fn create(g: &Generated) -> Self {
        todo!()
    }
}