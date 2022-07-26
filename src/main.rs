mod rbx;
mod cli;

use std::process;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let cli_args = Cli::parse();

    match cli_args.run().await {
        Ok(str) => {
            if let Some(s) = str {
                // log::info!("{}", s);
                println!("{}", s);
            }
        }
        Err(err) => {
            log::error!("{:?}", err);
            process::exit(1);
        }
    }
}
