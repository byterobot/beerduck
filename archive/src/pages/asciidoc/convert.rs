use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Error};

use crate::CONFIG;
use crate::pages::asciidoc::AsciiDoc;

/// Covert adoc file to html.
pub fn convert_adoc(adoc: &Path) -> Result<String, Error> {
    let temp_dir = CONFIG.workspace.temp.as_path();
    fs::create_dir_all(temp_dir)?;
    let doc = AsciiDoc::parse(&fs::read_to_string(adoc)?);
    let input = temp_dir.join(adoc.file_name().unwrap());
    fs::write(&input, doc.text())?;

    let name = input.file_name().ok_or(anyhow!("no file name for {:?}", adoc))?;
    let output = temp_dir.join(name).with_extension("html");

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