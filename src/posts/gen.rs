use std::collections::HashMap;

use anyhow::{anyhow, Error};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::posts::{Category, CategoryConfig, Generated, Preview, TextFile};
use crate::posts::page::Page;

pub fn gen_index(categories: &[Category]) -> Result<Generated, Error> {
    let mut items = vec![];
    for c in categories {
        for f in &c.files {
            items.push(build_preview(f, c)?);
        }
    }

    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(Generated { title: "".into(), items })
}

pub fn gen_categories(categories: &[Category]) -> Result<Generated, Error> {
    let mut ordered_categories: Vec<&Category> = categories.iter().map(|v| v).collect();
    ordered_categories.sort_by(|a, b|
        a.config.position.cmp(&b.config.position)
    );

    let mut items = vec![];
    for c in ordered_categories {
        let mut preview = Preview::default();
        preview.category = c.name.clone();
        preview.category_href = c.href();
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

    items.sort_by(|a, b| match config.date_asc {
        true => a.created_at.cmp(&b.created_at),
        _ =>b.created_at.cmp(&a.created_at),
    });

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
        href: f.href(),
        pin: config.pin.is_some() && config.pin.as_ref().unwrap().as_str() == file_name,
        created_at: page.created_at,
        summary: page.summary,
        category: c.name.clone(),
        category_href: c.href(),
    };
    Ok(preview)
}
