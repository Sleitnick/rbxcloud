use clap::{Args, Subcommand};
use rbxcloud::rbx::v2::{group::GroupId, RbxCloud};

#[derive(Debug, Subcommand)]
pub enum GroupCommands {
    /// Get info about the group
    Get {
        /// Group ID
        #[clap(short, long, value_parser)]
        group_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get the current shout and other metadata
    Shout {
        /// Group ID
        #[clap(short, long, value_parser)]
        group_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Only return the shout message string
        #[clap(short, long, value_parser, default_value_t = false)]
        only_message: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// List the roles of a group
    Roles {
        /// Group ID
        #[clap(short, long, value_parser)]
        group_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Max items returned per page
        #[clap(short, long, value_parser)]
        max_page_size: Option<u32>,

        /// Next page token
        #[clap(short, long, value_parser)]
        next_page_token: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Group {
    #[clap(subcommand)]
    command: GroupCommands,
}

impl Group {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            GroupCommands::Get {
                group_id,
                api_key,
                pretty,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let group = rbx_cloud.group(GroupId(group_id));
                let res = group.get_info().await;
                match res {
                    Ok(group_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&group_info)?
                        } else {
                            serde_json::to_string(&group_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            GroupCommands::Shout {
                group_id,
                pretty,
                only_message,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let group = rbx_cloud.group(GroupId(group_id));
                let res = group.get_shout().await;
                match res {
                    Ok(group_info) => {
                        if only_message {
                            return Ok(Some(group_info.content));
                        }
                        let r = if pretty {
                            serde_json::to_string_pretty(&group_info)?
                        } else {
                            serde_json::to_string(&group_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
            GroupCommands::Roles {
                group_id,
                api_key,
                pretty,
                max_page_size,
                next_page_token,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let group = rbx_cloud.group(GroupId(group_id));
                let res = group.list_roles(max_page_size, next_page_token).await;
                match res {
                    Ok(group_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&group_info)?
                        } else {
                            serde_json::to_string(&group_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
