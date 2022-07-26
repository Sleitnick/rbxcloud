mod rbx;
mod cli;

use std::process;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.run().await {
        Ok(str) => {
            if let Some(s) = str {
                log::info!("{}", s);
            }
        }
        Err(err) => {
            log::error!("{:?}", err);
            process::exit(1);
        }
    }
}
