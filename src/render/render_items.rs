use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Iter;

use anyhow::Error;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::pages::category::Category;
use crate::pages::page::Page;
use crate::pages::Pages;
use crate::render::{categories_target, category_target, category_url_path, home_target, page_url_path};
use crate::render::template::Template;
use crate::tpl::article::cast_date;
use crate::tpl::GLOBAL;
use crate::tpl::items::{Item, ItemsTpl};

pub fn render_items(pages: &Pages) -> Result<(), Error> {
    render_category(&pages)?;
    render_categories(&pages)?;
    render_index(&pages)?;
    Ok(())
}

fn render_category(pages: &Pages) -> Result<(), Error> {
    for c in pages.categories.values() {
        let mut items = build_pages(c, &pages.pages)?;
        items.sort_by(|a, b| {
            match a.pin {
                true => Ordering::Less,
                _ => match b.pin {
                    true => Ordering::Greater,
                    _ => match c.date_asc {
                        true => a.created_at.cmp(&b.created_at),
                        _ => b.created_at.cmp(&a.created_at),
                    }
                }
            }
        });

        let tpl = ItemsTpl { site: GLOBAL.deref(), title: c.name.clone(), items, };
        let path = category_target(&c.name);
        Template::Category.render_write(&tpl, &path)?;
    }
    Ok(())
}

fn render_categories(pages: &Pages) -> Result<(), Error> {
    let mut items = vec![];
    for c in pages.categories.values() {
        let mut item = Item::default();
        item.title = c.name.clone();
        item.href = category_url_path(c.alias.as_ref().unwrap_or(&c.name));
        items.push((item, c.position));
    }

    items.sort_by(|a, b| {
         match a.1 == b.1 {
             true => a.0.title.cmp(&b.0.title),
             _ => a.1.cmp(&b.1),
         }
    });

    let tpl = ItemsTpl {
        site: GLOBAL.deref(),
        title: "".to_string(),
        items: items.into_iter().map(|(i, _)| i).collect()
    };
    Template::Categories.render_write(&tpl, &categories_target())?;
    Ok(())
}

fn render_index(pages: &Pages) -> Result<(), Error> {
    let mut all_items = vec![];
    for c in pages.categories.values() {
        all_items.extend(build_pages(c, &pages.pages)?);
    }
    all_items.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    let tpl = ItemsTpl { site: GLOBAL.deref(), title: "".to_string(), items: all_items, };
    Template::Index.render_write(&tpl, &home_target())?;
    Ok(())
}

fn build_pages<'a>(c: &'a Category, pages: &'a HashMap<String, Page>)
    -> Result<Vec<Item<'a>>, Error> {
    let mut items = vec![];
    for name in &c.files {
        let p = pages.get(name).unwrap();
        let item = Item {
            title: p.title.clone(),
            href: page_url_path(name, false),
            category: c.name.clone(),
            category_href: category_url_path(c.alias.as_ref().unwrap_or(&c.name)),
            pin: c.pin.as_ref().map(|v| v == name).unwrap_or(false),
            created_at: cast_date(&p.created_at),
            summary: p.summary.as_ref()
        };
        items.push(item);
    }
    Ok(items)
}


#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    #[test]
    fn test() {
        let mut array = [8, 4, 1, 5, 6, 3, 0];
        let asc = false;
        array.sort_by(|a, b| {

            // 小到大
            if asc {
                match *a == 5 {
                    true => Ordering::Less,
                    _ => match *b == 5 {
                        true => Ordering::Greater,
                        _ => a.cmp(b)
                    },
                }
            } else {
                match *a == 5 {
                    true => Ordering::Less,
                    _ => match *b == 5 {
                        true => Ordering::Greater,
                        _ => b.cmp(a)
                    },
                }
            }
        });
        println!("{:?}", array);

    }
}
