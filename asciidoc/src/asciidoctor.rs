use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Error};

use config::{parent, workspace};

use crate::hybrid::Hybrid;

/// Convert adoc file to html.
pub fn convert(adoc: &Path) -> Result<String, Error> {
    let temp = parent().join(&workspace().temp);
    fs::create_dir_all(&temp)?;
    let doc = Hybrid::parse(&fs::read_to_string(adoc)?);
    let input = temp.join(adoc.file_name().unwrap());
    fs::write(&input, doc.text())?;

    let name = input.file_name().ok_or(anyhow!("no file name for {:?}", adoc))?;
    let output = temp.join(name).with_extension("html");

    let status = Command::new("asciidoctor")
        .args(["-r", "asciidoctor-html5s", "-b", "html5s"])
        .args(["-a", "linkcss"])
        .arg(input)
        .arg("-o")
        .arg(output.as_path())
        .status()?;

    match status.success() {
        true => Ok(fs::read_to_string(output.as_path())?),
        _ => Err(anyhow!("render file exit error")),
    }
}