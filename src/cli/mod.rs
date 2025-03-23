mod assets_cli;
mod datastore_cli;
mod experience_cli;
mod group_cli;
mod inventory_cli;
mod luau_execution_cli;
mod messaging_cli;
mod notification_cli;
mod ordered_datastore_cli;
mod place_cli;
mod subscription_cli;
mod universe_cli;
mod user_cli;

use clap::{Parser, Subcommand};
use inventory_cli::Inventory;
use luau_execution_cli::Luau;
use universe_cli::Universe;
use user_cli::User;

use self::{
    assets_cli::Assets, datastore_cli::DataStore, experience_cli::Experience, group_cli::Group,
    messaging_cli::Messaging, notification_cli::Notification,
    ordered_datastore_cli::OrderedDataStore, place_cli::Place, subscription_cli::Subscription,
};

#[derive(Debug, Parser)]
#[clap(name = "rbxcloud", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Access the Roblox Assets API
    Assets(Assets),

    /// Access the Roblox Experience API
    Experience(Experience),

    /// Access the Roblox Messaging API
    Messaging(Messaging),

    /// Access the Roblox DataStore API
    Datastore(DataStore),

    /// Access the Roblox OrderedDataStore API
    OrderedDatastore(OrderedDataStore),

    /// Access the Roblox user inventory API
    Inventory(Inventory),

    /// Access the Roblox Group API
    Group(Group),

    Luau(Luau),

    /// Access the Roblox Subscription API
    Subscription(Subscription),

    /// Access the Roblox Notification API
    Notification(Notification),

    /// Access the Roblox Place API
    Place(Place),

    /// Access the Roblox Universe API
    Universe(Universe),

    /// Access the Roblox User API
    User(User),
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            Command::Assets(command) => command.run().await,
            Command::Experience(command) => command.run().await,
            Command::Messaging(command) => command.run().await,
            Command::Datastore(command) => command.run().await,
            Command::OrderedDatastore(command) => command.run().await,
            Command::Group(command) => command.run().await,
            Command::Inventory(command) => command.run().await,
            Command::Luau(command) => command.run().await,
            Command::Subscription(command) => command.run().await,
            Command::Notification(command) => command.run().await,
            Command::Place(command) => command.run().await,
            Command::Universe(command) => command.run().await,
            Command::User(command) => command.run().await,
        }
    }
}
