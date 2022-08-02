use clap::{Args, Subcommand};

use rbxcloud::rbx::RbxCloud;

#[derive(Debug, Subcommand)]
pub enum MessagingCommands {
    /// Publish a message
    Publish {
        /// Message topic
        #[clap(short, long, value_parser)]
        topic: String,

        /// Message to send
        #[clap(short, long, value_parser)]
        message: String,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Messaging {
    #[clap(subcommand)]
    command: MessagingCommands,
}

impl Messaging {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            MessagingCommands::Publish {
                topic,
                message,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(api_key, universe_id);
                let messaging = rbx_cloud.messaging(&topic);
                let res = messaging.publish(message).await;
                match res {
                    Ok(()) => Ok(Some(format!("published message to topic {}", topic))),
                    Err(err) => Err(err),
                }
            }
        }
    }
}
