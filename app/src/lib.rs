#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]


use anyhow::Error;
use log::{info, LevelFilter};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TerminalMode, TermLogger};
use config::{parent, workspace};

pub async fn start_server() -> Result<(), Error> {
    let listener = listen_modified()?;
    let mut app = tide::new();

    // app.at("/").serve_dir(&CONFIG.workspace.publish)?;
    app.listen("0.0.0.0:2020").await?;
    Ok(())
}

pub fn listen_modified() -> Result<RecommendedWatcher, Error> {
    let mut watcher = RecommendedWatcher::new(|e: Result<Event, notify::Error>| {
        if let Ok(event) = e {
            // notify web
        }
    }, notify::Config::default())?;

    watcher.watch(&parent().join(&workspace().posts), RecursiveMode::Recursive)?;
    watcher.watch(&parent().join(&workspace().theme.self_dir), RecursiveMode::Recursive)?;
    watcher.watch(&parent().join(&workspace().assets.images), RecursiveMode::Recursive)?;
    Ok(watcher)
}



pub fn init_log() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            ConfigBuilder::new().add_filter_allow_str("adocnote").build(),
            TerminalMode::Mixed,
            ColorChoice::Auto)
    ]).unwrap()
}
