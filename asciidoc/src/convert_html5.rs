use std::fs;
use std::path::Path;
use std::process::{Command, ExitStatus};

use anyhow::Error;

use config::{parent, workspace};

use crate::hybrid::Hybrid;

pub fn convert(text: &str) -> Result<String, Error> {
    let dir = parent().join(&workspace().temp);
    fs::create_dir_all(&dir)?;
    let input = dir.join("input.adoc");
    let output = dir.join("output.html");
    fs::write(&input, Hybrid::parse(text).text())?;
    let _ = gem_html5(&input, &output)?;
    Ok(fs::read_to_string(&output)?)
}

fn gem_html5(input: &Path, output: &Path) -> Result<ExitStatus, Error> {
    let status = Command::new("asciidoctor")
        .args(["-r", "asciidoctor-html5s", "-b", "html5s"])
        .args(["-a", "linkcss"])
        .arg(input)
        .arg("-o")
        .arg(output)
        .status()?;
    Ok(status)
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use super::*;

    #[test]
    fn test() {
        let start = SystemTime::now();
        let file = "";
        let text = fs::read_to_string(file).unwrap();
        let html = convert(&text).unwrap();
        fs::write("output.html", html).unwrap();
        let end = SystemTime::now().duration_since(start).unwrap();
        println!("{:?}", end);
    }
}