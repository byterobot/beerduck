use std::collections::HashMap;

use anyhow::Error;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::posts::{Category, CategoryConfig, Generated, Preview, TextFile};
use crate::posts::page::Page;

pub fn gen_categories(categories: &[Category]) -> Result<Generated, Error> {
    let mut items = vec![];
    for c in categories {
        let preview = Preview {
            title: "".to_string(),
            pin: false,
            created_at: Default::default(),
            summary: None,
            link_name: "".to_string(),
            category: c.name.clone(),
            category_link_name: c.config.alias_name.clone().unwrap_or_else(|| c.name.clone()),
        };
        items.push(preview);
    }

    let g = Generated { title: "".to_string(), items };
    Ok(g)
}


pub fn gen_category(c: &Category) -> Result<Generated, Error> {
    let config = &c.config;
    let mut items = vec![];
    for f in &c.files {
        items.push(build_preview(f, c)?);
    }
    let g = Generated { title: c.name.clone(), items };
    Ok(g)
}

fn build_preview(f: &TextFile, c: &Category) -> Result<Preview, Error> {
    let config = &c.config;
    let file = &f.path;
    let page = Page::from_simple(file)?;
    let file_name = file.file_name().unwrap().to_str().unwrap();
    let preview = Preview {
        title: page.title,
        pin: config.pin.is_some() && config.pin.as_ref().unwrap().as_str() == file_name,
        created_at: page.created_at,
        summary: page.summary,
        link_name: REG.replace(file_name, "").to_string(),
        category: c.name.clone(),
        category_link_name: config.alias_name.clone().unwrap_or_else(|| c.name.clone()),
    };
    Ok(preview)
}

static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(adoc)$").unwrap());

