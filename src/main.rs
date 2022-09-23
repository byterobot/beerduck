use std::error::Error;
use adocnote::start_server;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    start_server().await?;
    Ok(())
}
