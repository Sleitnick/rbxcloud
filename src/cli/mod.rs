mod experience_cli;
mod messaging_cli;

use clap::{Parser, Subcommand};

use self::{experience_cli::{Experience}, messaging_cli::Messaging};

#[derive(Debug, Parser)]
#[clap(name = "Rbx Cloud", version, about, author)]
pub struct Cli {
	#[clap(subcommand)]
	pub command: Command,
}

impl Cli {
	pub async fn run(self) -> anyhow::Result<()> {
		match self.command {
			Command::Experience(command) => command.run().await,
			Command::Messaging(command) => command.run().await,
		}
	}
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(arg_required_else_help = true)]
	Experience(Experience),
	Messaging(Messaging),
}
