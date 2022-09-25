#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]

use anyhow::Error;
use log::info;

use crate::config::CONFIG;
use crate::render::{init, listen_posts};

pub(crate) mod config;
pub(crate) mod pages;
pub(crate) mod render;
pub(crate) mod template;

pub async fn start_server() -> Result<(), Error> {
    publish()?;
    info!("watching modify.");
    let w = listen_posts()?;
    info!("start server.");
    let mut app = tide::new();
    app.at("/").serve_dir(&CONFIG.workspace.publish)?;
    app.listen("0.0.0.0:1919").await?;
    Ok(())
}

pub fn publish() -> Result<(), Error> {
    info!("init...");
    init();
    info!("render...");
    render::render()?;
    Ok(())
}
