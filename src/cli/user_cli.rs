use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::RobloxUserId,
    v2::{
        user::{UserThumbnailFormat, UserThumbnailShape, UserThumbnailSize},
        Client,
    },
};

#[derive(Debug, Subcommand)]
pub enum UserCommands {
    /// Get user information
    Get {
        /// User ID
        #[clap(short, long, value_parser)]
        user_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Generate user thumbnail
    Thumbnail {
        /// User ID
        #[clap(short, long, value_parser)]
        user_id: u64,

        /// Thumbnail size
        #[clap(short, long, value_enum)]
        size: Option<UserThumbnailSize>,

        /// Thumbnail format
        #[clap(short, long, value_enum)]
        format: Option<UserThumbnailFormat>,

        /// Thumbnail shape
        #[clap(short = 'S', long, value_enum)]
        shape: Option<UserThumbnailShape>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct User {
    #[clap(subcommand)]
    command: UserCommands,
}

impl User {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            UserCommands::Get {
                user_id,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let user_client = client.user();
                let res = user_client.get_user(RobloxUserId(user_id)).await;
                match res {
                    Ok(universe_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&universe_info)?
                        } else {
                            serde_json::to_string(&universe_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            UserCommands::Thumbnail {
                user_id,
                size,
                format,
                shape,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let user_client = client.user();
                let res = user_client
                    .generate_thumbnail(RobloxUserId(user_id), size, format, shape)
                    .await;
                match res {
                    Ok(universe_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&universe_info)?
                        } else {
                            serde_json::to_string(&universe_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
