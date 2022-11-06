use std::env::args;
use std::error::Error;

use log::{error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TerminalMode, TermLogger};

use app::start_server;
use config::set_mode;
use data::publish::publish;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_log();
    if let Some(v) = args().nth(1) {
        match v.as_str() {
            "start" => {
                set_mode(true);
                start_server().await?;
            },
            "build" => {
                set_mode(false);
                publish()?;
            },
            _ => error!("unknown command!"),
        }
    }
    Ok(())
}

fn init_log() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            ConfigBuilder::new().add_filter_allow_str("app").build(),
            TerminalMode::Mixed,
            ColorChoice::Auto)
    ]).unwrap()
}
