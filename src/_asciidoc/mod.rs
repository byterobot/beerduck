use crate::asciidoc::doc::Doc;
use crate::asciidoc::header::Header;

mod doc;
pub mod header;

pub struct AsciiDoc {
    doc: Doc
}

impl AsciiDoc {
    pub fn from(text: &str) -> Self {
        Self { doc: Doc::from(text)}
    }

    pub fn head(&self) -> Option<&Header> {
        match &self.doc {
            Doc::Single(d) => d.head.as_ref(),
            Doc::Hybrid(_, d) => d.head.as_ref(),
        }
    }

    pub fn text(&self) -> &str {
        match &self.doc {
            Doc::Single(d) => &d.text,
            Doc::Hybrid(_, d) => &d.text,
        }
    }

    pub fn render(&self) -> String {
        todo!()
    }

}