use std::borrow::Cow;

use nipper::Document;
use once_cell::sync::Lazy;
use regex::Regex;

fn convert(xhtml: &str) -> String {
    let document = Document::from(xhtml);
    let selection = document.select("#content .paragraph");
    for mut item in selection.iter() {
        if let Some(child) = item.children().iter().next() {
            item.replace_with_selection(&child);
        }
    }

    let selection = document.select("#content .ulist li");
    for mut item in selection.iter() {
        if let Some(child) = item.children().iter().next() {
            item.set_html(inner_html(child.html().trim()));
        }
    }

    document.html().to_string()
}

fn inner_html(html: &str) -> String {
    let html = START_TAG.replace(html.trim(), "");
    END_TAG.replace(html.as_ref(), "").to_string()
}

static START_TAG: Lazy<Regex> = Lazy::new(|| Regex::new("^<[^>]+>").unwrap());
static END_TAG: Lazy<Regex> = Lazy::new(|| Regex::new("</[^>]+>$").unwrap());


#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test() {
        let temp = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp");
        let input = temp.join("xhtml.html");
        let output = temp.join("xhtml.modify.html");

        let xhtml = fs::read_to_string(&input).unwrap();
        let html5 = convert(&xhtml);
        fs::write(&output, &html5).unwrap();
    }
}