use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::{PlaceId, UniverseId},
    v2::{place::UpdatePlaceInfo, Client},
};

#[derive(Debug, Subcommand)]
pub enum PlaceCommands {
    /// Get Place information
    Get {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID
        #[clap(short, long, value_parser)]
        place_id: u64,

        /// Pretty-print the JSON response
        #[clap(long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update Place name
    UpdateName {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID
        #[clap(short, long, value_parser)]
        place_id: u64,

        /// New Place name
        #[clap(short, long, value_parser)]
        name: String,

        /// Pretty-print the JSON response
        #[clap(long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update Place description
    UpdateDescription {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID
        #[clap(short, long, value_parser)]
        place_id: u64,

        /// New Place description
        #[clap(short, long, value_parser)]
        description: String,

        /// Pretty-print the JSON response
        #[clap(long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update Place server size
    UpdateServerSize {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID
        #[clap(short, long, value_parser)]
        place_id: u64,

        /// New Place server size
        #[clap(short, long, value_parser)]
        server_size: i32,

        /// Pretty-print the JSON response
        #[clap(long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Place {
    #[clap(subcommand)]
    command: PlaceCommands,
}

impl Place {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            PlaceCommands::Get {
                universe_id,
                place_id,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let place_client = client.place(UniverseId(universe_id), PlaceId(place_id));
                let res = place_client.get().await;
                match res {
                    Ok(place_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&place_info)?
                        } else {
                            serde_json::to_string(&place_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            PlaceCommands::UpdateName {
                universe_id,
                place_id,
                name,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let place_client = client.place(UniverseId(universe_id), PlaceId(place_id));
                let res = place_client
                    .update(
                        "displayName".to_string(),
                        UpdatePlaceInfo {
                            path: None,
                            create_time: None,
                            update_time: None,
                            display_name: Some(name),
                            description: None,
                            server_size: None,
                        },
                    )
                    .await;
                match res {
                    Ok(place_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&place_info)?
                        } else {
                            serde_json::to_string(&place_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            PlaceCommands::UpdateDescription {
                universe_id,
                place_id,
                description,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let place_client = client.place(UniverseId(universe_id), PlaceId(place_id));
                let res = place_client
                    .update(
                        "description".to_string(),
                        UpdatePlaceInfo {
                            path: None,
                            create_time: None,
                            update_time: None,
                            display_name: None,
                            description: Some(description),
                            server_size: None,
                        },
                    )
                    .await;
                match res {
                    Ok(place_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&place_info)?
                        } else {
                            serde_json::to_string(&place_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            PlaceCommands::UpdateServerSize {
                universe_id,
                place_id,
                server_size,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let place_client = client.place(UniverseId(universe_id), PlaceId(place_id));
                let res = place_client
                    .update(
                        "serverSize".to_string(),
                        UpdatePlaceInfo {
                            path: None,
                            create_time: None,
                            update_time: None,
                            display_name: None,
                            description: None,
                            server_size: Some(server_size),
                        },
                    )
                    .await;

                match res {
                    Ok(place_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&place_info)?
                        } else {
                            serde_json::to_string(&place_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
