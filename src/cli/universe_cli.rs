use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::UniverseId,
    v2::{universe::UpdateUniverseInfo, Client},
};

#[derive(Debug, Subcommand)]
pub enum UniverseCommands {
    /// Get universe information
    Get {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Restart servers
    Restart {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update Universe name
    UpdateName {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// New Universe name
        #[clap(short, long, value_parser)]
        name: String,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update Universe description
    UpdateDescription {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// New Universe description
        #[clap(short, long, value_parser)]
        description: String,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Universe {
    #[clap(subcommand)]
    command: UniverseCommands,
}

impl Universe {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            UniverseCommands::Get {
                universe_id,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let universe_client = client.universe(UniverseId(universe_id));
                let res = universe_client.get().await;
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

            UniverseCommands::Restart {
                universe_id,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let universe_client = client.universe(UniverseId(universe_id));
                let res = universe_client.restart_servers().await;
                match res {
                    Ok(()) => Ok(Some("servers restarted".to_string())),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            UniverseCommands::UpdateName {
                universe_id,
                name,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let universe_client = client.universe(UniverseId(universe_id));
                let res = universe_client
                    .update(
                        "displayName".to_string(),
                        UpdateUniverseInfo {
                            path: None,
                            create_time: None,
                            update_time: None,
                            display_name: Some(name),
                            description: None,
                            user: None,
                            group: None,
                            visibility: None,
                            facebook_social_link: None,
                            twitter_social_link: None,
                            youtube_social_link: None,
                            twitch_social_link: None,
                            discord_social_link: None,
                            roblox_group_social_link: None,
                            guilded_social_link: None,
                            voice_chat_enabled: None,
                            age_rating: None,
                            private_server_price_robux: None,
                            desktop_enabled: None,
                            mobile_enabled: None,
                            tablet_enabled: None,
                            console_enabled: None,
                            vr_enabled: None,
                        },
                    )
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

            UniverseCommands::UpdateDescription {
                universe_id,
                description,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let universe_client = client.universe(UniverseId(universe_id));
                let res = universe_client
                    .update(
                        "description".to_string(),
                        UpdateUniverseInfo {
                            path: None,
                            create_time: None,
                            update_time: None,
                            display_name: None,
                            description: Some(description),
                            user: None,
                            group: None,
                            visibility: None,
                            facebook_social_link: None,
                            twitter_social_link: None,
                            youtube_social_link: None,
                            twitch_social_link: None,
                            discord_social_link: None,
                            roblox_group_social_link: None,
                            guilded_social_link: None,
                            voice_chat_enabled: None,
                            age_rating: None,
                            private_server_price_robux: None,
                            desktop_enabled: None,
                            mobile_enabled: None,
                            tablet_enabled: None,
                            console_enabled: None,
                            vr_enabled: None,
                        },
                    )
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
