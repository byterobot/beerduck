use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use futures_await_test::async_test;

use asciidoc::hybrid::Hybrid;

#[async_test]
async fn test() {
    let path = "";
    let text = fs::read_to_string(path).unwrap();
    let hyper = Hybrid::parse(&text);

    let temp = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp");
    println!("{:?}", temp);

    // convert xhtml
    let xhtml = asciidoc::convert::convert(hyper.text()).unwrap();
    fs::write(temp.join("xhtml.html"), &xhtml).unwrap();

    let xhtml_modify = asciidoc::convert(hyper.text()).unwrap();
    fs::write(temp.join("xhtml.modify.html"), &xhtml_modify).unwrap();

    let input = temp.join("ch02.adoc");
    fs::write(&input, hyper.text()).unwrap();
    // convert html5
    gem_html5(&input, &temp.join("html5.html"));
}

fn gem_html5(input: &Path, output: &Path) {
    let _status = Command::new("asciidoctor")
        .args(["-r", "asciidoctor-html5s", "-b", "html5s"])
        .args(["-a", "linkcss"])
        .arg(input)
        .arg("-o")
        .arg(output)
        .status().unwrap();
}