pub struct Hybrid {
    source: Option<String>,
    target: String,
}

impl Hybrid {
    pub fn parse(text: &str) -> Self {
        if !(text.starts_with("+\n") || text.starts_with("-\n") || text.starts_with("=\n")) {
            return Self { source: None, target: text.into(), };
        }

        let (mut origin, mut target) = (String::new(), String::new());
        let mut way = (&mut origin, Some(&mut target));
        for line in text.split("\n") {
            match line {
                "-" => way = (&mut origin, None),
                "+" => way = (&mut target, None),
                "=" => way = (&mut origin, Some(&mut target)),
                _ => {}
            }
            let line = match line {
                "-" | "+" | "=" => "",
                _ => line,
            };

            way.0.push_str(&format!("{}\n", line));
            if let Some(r) = way.1.as_deref_mut() {
                r.push_str(&format!("{}\n", line));
            }
        }

        Self { source: Some(origin.trim_start().into()), target: target.trim_start().into(), }
    }

    pub fn text(&self) -> &str {
        &self.target
    }

}
