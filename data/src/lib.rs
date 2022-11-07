#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]

use std::path::Path;

use anyhow::Error;
use log::error;

use config::{parent, site, workspace};

pub mod page;
pub mod publish;
pub mod template;

pub fn endpoint(path: &str) -> Option<String> {
    // println!("path: {}", path);
    let endpoint = Endpoint::parse(path);
    // println!("endpoint: {:?}", endpoint);
    match publish::get(&endpoint) {
        Ok(v) => Some(v),
        Err(e) => {
            error!("{}", e);
            None
        }
    }
}

#[derive(Debug)]
pub enum Endpoint {
    Page(String, String),
    // category folder name
    Category(String),
    // categories index
    Categories,
    Index,
    About,
    Unknown(String),
}

impl Endpoint {
    fn parse(path: &str) -> Self {
        let mut parts: Vec<&str> = path.split("/").filter(|v| !v.is_empty()).collect();
        if let Some(v) = parts.last_mut() {
            *v = file_stem(v);
        }
        let mut endpoint = match parts.len() {
            0 => Self::Index,
            1 => one_part(&parts),
            2 => two_parts(&parts),
            _ => multiple_parts(&parts),
        };
        if let Self::Unknown(v) = &mut endpoint {
            *v = path.to_string();
        }
        endpoint
    }
}

fn one_part(parts: &[&str]) -> Endpoint {
    match *parts.first().unwrap() {
        "index" => Endpoint::Index,
        "about" => Endpoint::About,
        "categories" => Endpoint::Categories,
        _ => Endpoint::Unknown("".into()),
    }
}

fn two_parts(parts: &[&str]) -> Endpoint {
    let first = *parts.first().unwrap();
    let last = *parts.last().unwrap();
    match first {
        "categories" => {
            match find_category(last) {
                Ok(Some(name)) => Endpoint::Category(name.to_string()),
                Ok(None) => Endpoint::Unknown("".into()),
                Err(e) => {
                    error!("find category {} failed, {}", last, e);
                    Endpoint::Unknown("".into())
                },
            }
        },
        _ => multiple_parts(parts),
    }
}

fn multiple_parts(parts: &[&str]) -> Endpoint {
    match find_page(parts.last().unwrap()) {
        Ok(Some(v)) => Endpoint::Page(v.0, v.1),
        Ok(None) => Endpoint::Unknown(parts.join("/").to_string()),
        Err(e) => {
            error!("find path {:?} failed, {}", parts, e);
            Endpoint::Unknown("".into())
        }
    }
}

fn find_page(name: &str) -> Result<Option<(String, String)>, Error> {
    let (_, paths) = publish::categories::create()?;
    for path in paths {
        for file in publish::category::files(&path)? {
            if file.file_stem().unwrap().to_str().unwrap() == name {
                let category = path.file_stem().unwrap().to_str().unwrap();
                return Ok(Some((category.to_string(), name.to_string())));
            }
        }
    }
    Ok(None)
}

fn find_category(name: &str) -> Result<Option<String>, Error> {
    let (_, paths) = publish::categories::create()?;
    for path in paths {
        if path.file_stem().unwrap().to_str().unwrap() == name {
            return Ok(Some(name.to_string()));
        }
    }
    Ok(None)
}

fn file_stem(text: &str) -> &str {
    Path::new(text).file_stem().unwrap().to_str().unwrap()
}
