use chrono::NaiveDate;
use tl::VDom;

pub use images::*;

mod images;

pub fn get_title(doc: &VDom) -> Option<String> {
    let title = doc.query_selector("title")?
        .next()?
        .get(doc.parser())?
        .inner_text(doc.parser()).to_string();
    Some(title)
}

pub fn get_author(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("author")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    Some(v.as_ref().to_string())
}

pub fn get_lang(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"html[lang]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("lang")??;
    let b = String::from_utf8(a.as_bytes().to_vec()).ok()?;
    Some(b)
}

pub fn get_keywords(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"meta[name="keywords"]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("content")??;
    String::from_utf8(a.as_bytes().to_vec()).ok()
}

pub fn get_description(doc: &VDom) -> Option<String> {
    let a = doc.query_selector(r#"meta[name="description"]"#)?
        .next()?.get(doc.parser())?.as_tag()?.attributes().get("content")??;
    String::from_utf8(a.as_bytes().to_vec()).ok()
}

pub fn get_toc(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("toc")?
        .get(doc.parser())?
        .as_tag()?
        .query_selector(doc.parser(),"ol")?
        .next()?
        .get(doc.parser())?
        .outer_html(doc.parser());
    Some(v.to_string())
}

pub fn get_date(doc: &VDom) -> Option<NaiveDate> {
    let v = doc.get_element_by_id("revdate")?
        .get(doc.parser())?
        .inner_text(doc.parser());
    let date = NaiveDate::parse_from_str(v.as_ref(), "%Y-%m-%d")
        .expect("error date format, must `yyyy-mm-dd` format");
    Some(date)
}

pub fn get_content(doc: &VDom) -> Option<String> {
    let v = doc.get_element_by_id("content")?
        .get(doc.parser())?
        .outer_html(doc.parser());
    Some(v.trim().to_string())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let doc = tl::parse(include_str!("/Users/me/ch03.html"), tl::ParserOptions::new()).unwrap();
        let v = doc.get_element_by_id("toc").unwrap()
            .get(doc.parser()).unwrap()
            .as_tag().unwrap()
            .query_selector(doc.parser(),"ol").unwrap()
            .next().unwrap()
            .get(doc.parser()).unwrap()
            .outer_html(doc.parser());

        println!("{}", v);
    }
}