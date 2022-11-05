use std::env::args;
use std::error::Error;

use log::error;

use app::{init_log, start_server};
use data::publish::publish;

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
