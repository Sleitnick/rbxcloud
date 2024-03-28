use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::{RobloxUserId, UniverseId},
    v2::Client,
};

#[derive(Debug, Subcommand)]
pub enum NotificationCommands {
    Send {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// User ID
        #[clap(short, long, value_parser)]
        user_id: u64,

        /// Payload
        #[clap(short = 'P', long, value_parser)]
        payload: String,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Notification {
    #[clap(subcommand)]
    command: NotificationCommands,
}

impl Notification {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            NotificationCommands::Send {
                universe_id,
                user_id,
                payload,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let notification_client = client.notification(UniverseId(universe_id));

                let notification = serde_json::from_str::<
                    rbxcloud::rbx::v2::notification::Notification,
                >(&payload)?;

                let res = notification_client
                    .send(RobloxUserId(user_id), notification)
                    .await;

                match res {
                    Ok(subscription_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&subscription_info)?
                        } else {
                            serde_json::to_string(&subscription_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
