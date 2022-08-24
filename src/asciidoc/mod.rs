use crate::asciidoc::doc::Doc;
use crate::asciidoc::head::Head;

mod doc;
pub mod head;

pub struct AsciiDoc {
    doc: Doc
}

impl AsciiDoc {
    pub fn from(text: &str) -> Self {
        Self { doc: Doc::from(text)}
    }

    pub fn head(&self) -> Option<&Head> {
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