pub mod convert;

pub struct AsciiDoc {
    source: Option<String>,
    target: String,
}

impl AsciiDoc {
    pub fn from(text: &str) -> Self {
        let text = text.trim();
        if !text.starts_with(r"^\[[-+=]\]$") {
            return Self { source: None, target: text.to_string() }
        }

        let mut source = String::new();
        let mut target = String::new();
        for (v, t) in Self::split_hybrid(text) {
            match t {
                TextType::Source => source.push_str(&v),
                TextType::Target => target.push_str(&v),
                TextType::Share => {
                    source.push_str(&v);
                    target.push_str(&v);
                }
            }
        }

        Self { source: Some(source), target }
    }

    pub fn text(&self) -> &str {
        &self.target
    }

    fn split_hybrid(text: &str) -> Vec<(String, TextType)> {
        let mut vec = vec![];
        for line in text.split("(\r\n|\r|\n)") {
            match TextType::match_type(line) {
                Some(v) => vec.push((String::new(), v)),
                _ => {
                    match vec.last_mut() {
                        Some((t, _)) => {
                            t.push_str(line);
                            t.push('\n');
                        },
                        _ => {
                            panic!("Translate file syntax error, \
                            must start with `[-]`, `[+]` or `[=]`");
                        }
                    }
                },
            }
        }
        vec
    }

}


#[derive(Copy, Clone, Eq, PartialEq)]
enum TextType {
    Source, Target, Share
}

impl TextType {
    fn match_type(line: &str) -> Option<TextType> {
        match line.trim() {
            "[-]" => Some(TextType::Source),
            "[+]" => Some(TextType::Target),
            "[=]" => Some(TextType::Share),
            _ => None,
        }
    }
}