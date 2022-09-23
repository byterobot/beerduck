#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]

use anyhow::Error;

use crate::config::CONFIG;
use crate::render::{init, render, render_reload};

pub(crate) mod asciidoc;
pub(crate) mod config;
pub(crate) mod convert;
pub(crate) mod pages;
pub(crate) mod render;
pub(crate) mod tpl;

pub async fn start_server() -> Result<(), Error> {
    init();
    render()?;
    let w = render_reload()?;

    let mut app = tide::new();
    app.at("/").serve_dir(&CONFIG.workspace.publish)?;
    app.listen("0.0.0.0:1919").await?;
    Ok(())
}


