mod datastore_cli;
mod experience_cli;
mod messaging_cli;
mod ordered_datastore_cli;

use clap::{Parser, Subcommand};

use self::{
    datastore_cli::DataStore, experience_cli::Experience, messaging_cli::Messaging,
    ordered_datastore_cli::OrderedDataStore,
};

#[derive(Debug, Parser)]
#[clap(name = "rbxcloud", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Access the Roblox Experience API
    Experience(Experience),

    /// Access the Roblox Messaging API
    Messaging(Messaging),

    /// Access the Roblox DataStore API
    Datastore(DataStore),

    /// Access the Roblox OrderedDataStore API
    OrderedDatastore(OrderedDataStore),
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            Command::Experience(command) => command.run().await,
            Command::Messaging(command) => command.run().await,
            Command::Datastore(command) => command.run().await,
            Command::OrderedDatastore(command) => command.run().await,
        }
    }
}
