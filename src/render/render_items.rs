use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Deref;
use anyhow::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::convert::Template;
use crate::pages::category::Category;
use crate::pages::page::Page;

use crate::pages::Pages;
use crate::render::{category_target, page_url_path};
use crate::tpl::article::cast_date;
use crate::tpl::GLOBAL;
use crate::tpl::items::{Item, ItemsTpl};

pub fn render_items(pages: &Pages) -> Result<(), Error> {
    // 创建 category list, categories, index.
    for c in pages.categories.values() {
        render_category(c, &pages.pages)?;
    }


    todo!()
}

fn render_category(c: &Category, pages: &HashMap<String, Page>) -> Result<(), Error> {
    let mut items = vec![];
    for name in &c.files {
        let p = pages.get(name).unwrap();
        let item = Item {
            title: p.title.clone(),
            href: page_url_path(name, false),
            category: "".to_string(),
            category_href: "".to_string(),
            pin: c.pin.as_ref().map(|v| v == name).unwrap_or(false),
            created_at: cast_date(&p.created_at),
            summary: p.summary.as_ref()
        };
        items.push(item);
    }

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
    Ok(())
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
