use nipper::{Document, Selection};
use once_cell::sync::Lazy;
use regex::Regex;
use tendril::StrTendril;

pub fn convert(xhtml: &str) -> String {
    let document = Document::from(xhtml);
    header(&document);

    let content = document.select("#content");
    figure(&content);
    paragraph(&content);
    block(&content);
    section(&content);

    document.html().to_string()
}

fn section(content: &Selection) {
    for mut sec in content.select(".sect2").iter() {
        let inner = inner_html(sec.html());
        let html = format!(r#"<section class="doc-section level-2">{}</section>"#, inner);
        sec.replace_with_html(html);
    }
    for mut sec in content.select(".sect1").iter() {
        let inner = inner_html(sec.html());
        let html = format!(r#"<section class="doc-section level-1">{}</section>"#, inner);
        sec.replace_with_html(html);
    }
    for mut sec in content.select(".sectionbody").iter() {
        sec.replace_with_html(inner_html(sec.html()));
    }
}

fn figure(content: &Selection) {
    for mut block in content.select(".imageblock").iter() {
        let img = block.select("img").html();
        let title = inner_html(block.select(".title").html());
        let figure = format!(r#"<figure class="image-block">{}<figcaption>{}</figcaption></figure>"#, img, title);
        block.replace_with_html(figure);
    }
}

fn block(content: &Selection) {
    for mut block in content.select(".listingblock, .literalblock").iter() {
        let sub = inner_html(block.select(".content").html());
        block.set_html(sub);

        if block.has_class("listingblock") {
            block.remove_class("listingblock");
            block.add_class("listing-block");
        } else if block.has_class("literalblock") {
            block.remove_class("literalblock");
            block.add_class("literal-block");
        }
    }
}

fn paragraph(content: &Selection) {
    for mut item in content.select(".paragraph").iter() {
        if let Some(child) = item.children().iter().next() {
            item.replace_with_selection(&child);
        }
    }

    for mut item in content.select(".ulist li").iter() {
        if let Some(child) = item.children().iter().next() {
            item.set_html(inner_html(child.html()));
        }
    }
}

fn header(document: &Document) {
    let header = document.select("#header");
    let mut date = header.select("#revdate");
    let tag = format!(r#"<time id="revdate" datetime="{0}">{0}</time>"#, date.text());
    date.replace_with_html(tag);

    let mut toc_title = header.select("#toctitle");
    let tag = format!(r#"<h2 id="toc-title">{}</h2>"#, inner_html(toc_title.html()));
    toc_title.replace_with_html(tag);

    let mut level2 = header.select(".sectlevel2");
    level2.remove_class("sectlevel2");
    level2.add_class("toc-list level-2");

    let mut level1 = header.select(".sectlevel1");
    level1.remove_class("sectlevel1");
    level1.add_class("toc-list level-1");

    let mut toc = header.select("#toc");
    let nav = format!(r#"<nav id="toc" class="toc">{}</nav>"#, inner_html(toc.html()));
    toc.replace_with_html(nav);
}

fn inner_html(html: StrTendril) -> String {
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