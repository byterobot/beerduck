use std::env::args;
use std::error::Error;

use log::{error, LevelFilter};
use simplelog::*;

use adocnote::{publish, start_server};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_log();
    if let Some(v) = args().nth(1) {
        match v.as_str() {
            "start" => start_server().await?,
            "build" => publish()?,
            _ => error!("unknown command!"),
        }
    }
    Ok(())
}

fn init_log() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            ConfigBuilder::new().add_filter_allow_str("adocnote").build(),
            TerminalMode::Mixed,
            ColorChoice::Auto)
    ]).unwrap()
}
