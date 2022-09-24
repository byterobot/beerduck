use std::fs;

use anyhow::Error;

use crate::config::CONFIG;
use crate::convert::Template;
use crate::pages::Pages;
use crate::render::{page_target, remove_absolute, resolve_image_path};
use crate::tpl::article::ArticleTpl;

pub fn render_pages(pages: &Pages) -> Result<(), Error> {
    for name in pages.pages.keys() {
        render_page(pages, name)?;
    }
    Ok(())
}

pub fn render_page(pages: &Pages, name: &str) -> Result<(), Error> {
    if let Some(page) = pages.pages.get(name) {
        let tpl = match pages.categories_name.get(name) {
            Some(c) =>
                ArticleTpl::from(page, pages.categories.get(c).unwrap()),
            _ => ArticleTpl::single(page),
        };

        // copy_images(&page.images)?;

        let is_single = !pages.categories_name.contains_key(name);
        let path = page_target(name, is_single);
        fs::create_dir_all(path.parent().unwrap())?;

        match pages.categories_name.contains_key(name) {
            true => Template::Article.render_write(&tpl, &path)?,
            _ => Template::About.render_write(&tpl, &path)?,
        }
    }

    Ok(())
}

fn copy_images(images: &[String]) -> Result<(), Error> {
    let static_ = &CONFIG.workspace.static_;
    let publish = &CONFIG.workspace.publish;
    for src in images {
        let src = remove_absolute(src);
        let s = static_.join(src.as_ref());
        let t = publish.join(resolve_image_path(src.as_ref()));
        fs::create_dir_all(&t.parent().unwrap())?;
        fs::copy(&s, &t).map_err(|e| {
            println!("error: {}, path: {}", e, s.to_str().unwrap())
        });
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use regex::Regex;

    #[test]
    fn test() {
        let p = PathBuf::from("/abc");
        let p = p.join("");
        let p = p.join("cde");
        println!("{:?}", p);
    }

    #[test]
    fn test_reg() {
        let reg = Regex::new("^/").unwrap();
        let t = "/abc";
        let n = reg.replace(t, "");
        println!("{}", n.as_ref())
    }
}