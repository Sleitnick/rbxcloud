use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum LuauExecutionCommands {}

#[derive(Debug, Args)]
pub struct Luau {
    #[clap(subcommand)]
    command: LuauExecutionCommands,
}

impl Luau {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {}
    }
}
