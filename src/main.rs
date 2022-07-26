mod rbx;
mod cli;
mod util;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    args.run().await.unwrap();
}
