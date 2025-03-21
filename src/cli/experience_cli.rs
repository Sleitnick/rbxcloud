use clap::{Args, Subcommand, ValueEnum};

use rbxcloud::rbx::{
    types::{PlaceId, UniverseId},
    v1::{PublishVersionType, RbxCloud},
};

#[derive(Debug, Subcommand)]
pub enum ExperienceCommands {
    /// Publish an experience
    Publish {
        /// Filename (full or relative) of the RBXL file
        #[clap(short, long, value_parser)]
        filename: String,

        /// Place ID of the experience
        #[clap(short = 'i', long, value_parser)]
        place_id: u64,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Version type
        #[clap(short = 't', long, value_enum)]
        version_type: VersionType,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Experience {
    #[clap(subcommand)]
    command: ExperienceCommands,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum VersionType {
    Saved,
    Published,
}

impl Experience {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            ExperienceCommands::Publish {
                filename,
                place_id,
                universe_id,
                version_type,
                pretty,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let publish_version_type = match version_type {
                    VersionType::Published => PublishVersionType::Published,
                    VersionType::Saved => PublishVersionType::Saved,
                };
                let res = rbx_cloud
                    .experience(UniverseId(universe_id), PlaceId(place_id))
                    .publish(&filename, publish_version_type)
                    .await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(err.into()),
                }
            }
        }
    }
}
