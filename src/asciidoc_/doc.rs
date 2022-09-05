use crate::asciidoc::header::Header;

pub enum Doc {
    Single(AdocInner), Hybrid(AdocInner, AdocInner),
}

impl Doc {
    pub fn from(text: &str) -> Self {
        let text = text.trim();
        if !text.starts_with(r"^\[[-+=]\]$") {
            return Doc::Single(AdocInner::from(text))
        }

        let mut source = String::new();
        let mut target = String::new();
        for (v, t) in Self::parse_hybrid(text) {
            match t {
                TextType::Source => source.push_str(&v),
                TextType::Target => target.push_str(&v),
                TextType::Share => {
                    source.push_str(&v);
                    target.push_str(&v);
                }
            }
        }

        Doc::Hybrid(
            AdocInner::from(&source),
            AdocInner::from(&target),
        )
    }

    fn parse_hybrid(text: &str) -> Vec<(String, TextType)> {
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

pub struct AdocInner {
    pub head: Option<Header>,
    pub text: String,
}

impl AdocInner {
    fn from(text: &str) -> Self {
        let text = text.trim();
        if !text.starts_with("---") {
            return Self { head: None, text: text.to_string() }
        }

        let mut head = String::new();
        let mut body = String::new();
        let mut mark = false;
        for line in text.split('\n') {
            if line.trim() == "---" {
                mark = !mark;
                continue;
            }
            if mark {
                head.push_str(&line.replacen(':', "=", 1)); // for toml parse
                head.push('\n');
            } else {
                body.push_str(line);
                body.push('\n');
            }
        }

        let head = match head.is_empty() {
            true => Header::new(),
            _ => Header::from(&head),
        };

        Self { head: Some(head), text: text.trim().to_string() }
    }
}


