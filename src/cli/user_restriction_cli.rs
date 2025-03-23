use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::{PlaceId, RobloxUserId, UniverseId},
    v2::{Client, UserRestrictionParams},
};

#[derive(Debug, Subcommand)]
pub(crate) enum UserRestrictionCommands {
    /// Get user restriction information
    Get {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// User ID
        #[clap(short = 'U', long, value_parser)]
        user_id: u64,

        /// Place ID
        #[clap(short = 'P', long, value_parser)]
        place_id: Option<u64>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update user restriction information
    Update {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// User ID
        #[clap(short = 'U', long, value_parser)]
        user_id: u64,

        /// Place ID
        #[clap(short = 'P', long, value_parser)]
        place_id: Option<u64>,

        /// Restriction active
        #[clap(short = 'A', long, value_parser)]
        active: Option<bool>,

        /// Restriction duration (seconds)
        #[clap(short, long, value_parser)]
        duration: Option<u64>,

        /// Private reason
        #[clap(short = 'r', long, value_parser)]
        private_reason: Option<String>,

        /// Display reason
        #[clap(short = 'D', long, value_parser)]
        display_reason: Option<String>,

        /// Exclude alternate accounts
        #[clap(short, long, value_parser)]
        exclude_alts: Option<bool>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// List user restrictions
    List {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID
        #[clap(short = 'P', long, value_parser)]
        place_id: Option<u64>,

        /// Max page size
        #[clap(short = 's', long, value_parser)]
        page_size: Option<u32>,

        /// Next page token
        #[clap(short, long, value_parser)]
        token: Option<String>,

        /// Filter
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// List user restriction logs
    Logs {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID
        #[clap(short = 'P', long, value_parser)]
        place_id: Option<u64>,

        /// Max page size
        #[clap(short = 's', long, value_parser)]
        page_size: Option<u32>,

        /// Next page token
        #[clap(short, long, value_parser)]
        token: Option<String>,

        /// Filter
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub(crate) struct UserRestriction {
    #[clap(subcommand)]
    command: UserRestrictionCommands,
}

impl UserRestriction {
    pub(crate) async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            UserRestrictionCommands::Get {
                universe_id,
                user_id,
                place_id,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let user_restriction_client = client.user_restriction(UniverseId(universe_id));
                let res = user_restriction_client
                    .get_user_restriction(
                        RobloxUserId(user_id),
                        place_id.and_then(|id| Some(PlaceId(id))),
                    )
                    .await;
                match res {
                    Ok(info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&info)?
                        } else {
                            serde_json::to_string(&info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            UserRestrictionCommands::Update {
                universe_id,
                user_id,
                place_id,
                active,
                duration,
                private_reason,
                display_reason,
                exclude_alts,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let mut user_restriction_client = client.user_restriction(UniverseId(universe_id));
                let res = user_restriction_client
                    .update_user_restriction(&UserRestrictionParams {
                        user_id: RobloxUserId(user_id),
                        place_id: place_id.and_then(|id| Some(PlaceId(id))),
                        active,
                        duration,
                        private_reason,
                        display_reason,
                        exclude_alt_accounts: exclude_alts,
                    })
                    .await;
                match res {
                    Ok(info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&info)?
                        } else {
                            serde_json::to_string(&info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            UserRestrictionCommands::List {
                universe_id,
                place_id,
                page_size,
                token,
                filter,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let user_restriction_client = client.user_restriction(UniverseId(universe_id));
                let res = user_restriction_client
                    .list_user_restrictions(
                        place_id.and_then(|id| Some(PlaceId(id))),
                        page_size,
                        filter,
                        token,
                    )
                    .await;
                match res {
                    Ok(info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&info)?
                        } else {
                            serde_json::to_string(&info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            UserRestrictionCommands::Logs {
                universe_id,
                place_id,
                page_size,
                token,
                filter,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let user_restriction_client = client.user_restriction(UniverseId(universe_id));
                let res = user_restriction_client
                    .list_user_restriction_logs(
                        place_id.and_then(|id| Some(PlaceId(id))),
                        page_size,
                        filter,
                        token,
                    )
                    .await;
                match res {
                    Ok(info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&info)?
                        } else {
                            serde_json::to_string(&info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
