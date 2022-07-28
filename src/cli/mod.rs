mod experience_cli;
mod messaging_cli;
mod datastore_cli;

use clap::{Parser, Subcommand};

use self::{experience_cli::Experience, messaging_cli::Messaging, datastore_cli::DataStore};

#[derive(Debug, Parser)]
#[clap(name = "Rbx Cloud", version, about, author)]
pub struct Cli {
	#[clap(subcommand)]
	pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(arg_required_else_help = true)]
	Experience(Experience),
	Messaging(Messaging),
	Datastore(DataStore),
}

impl Cli {
	pub async fn run(self) -> anyhow::Result<Option<String>> {
		match self.command {
			Command::Experience(command) => command.run().await,
			Command::Messaging(command) => command.run().await,
			Command::Datastore(command) => command.run().await,
		}
	}
}
