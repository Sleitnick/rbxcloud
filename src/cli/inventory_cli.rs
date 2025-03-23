use clap::{Args, Subcommand};
use rbxcloud::rbx::v2::Client;

#[derive(Debug, Subcommand)]
pub enum InventoryCommands {}

#[derive(Debug, Args)]
pub struct Inventory {
    #[clap(subcommand)]
    command: InventoryCommands,
}

impl Inventory {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {}
    }
}
