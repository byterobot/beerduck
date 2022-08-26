use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Error};

pub async fn render_file(file: &Path) -> Result<PathBuf, Error> {
    let name = file.file_name().ok_or(anyhow!("no file name for {:?}", file))?;
    let status = Command::new("asciidoctor")
        .arg(name)
        .status()?;

    match status.success() {
        true => Ok(file.with_file_name(name).with_extension("html")),
        _ => Err(anyhow!("render file exit error")),
    }
}

#[cfg(test)]
mod test {
    use std::env::current_dir;
    use std::fs;

    use futures_await_test::async_test;

    use crate::files::render::render_file;

    #[async_test]
    async fn test() {
        let a = current_dir().unwrap();
        println!("current: {:?}", a);

        let input = a.as_path().join("test.adoc");
        println!("input: {:?}", input.as_path());

        let output = render_file(&input).await.unwrap();
        println!("output: {:?}", output.as_path());
    }
}