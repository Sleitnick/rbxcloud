use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::UniverseId,
    v2::{subscription::SubscriptionView, Client},
};

#[derive(Debug, Subcommand)]
pub enum SubscriptionCommands {
    /// Get information about a subscription
    Get {
        /// Universe ID
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Subscription product ID
        #[clap(short = 'S', long, value_parser)]
        product: String,

        /// Subscription ID
        #[clap(short, long, value_parser)]
        subscription: String,

        /// View type
        #[clap(short, long, value_enum)]
        view: Option<SubscriptionView>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Subscription {
    #[clap(subcommand)]
    command: SubscriptionCommands,
}

impl Subscription {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            SubscriptionCommands::Get {
                universe_id,
                product,
                subscription,
                view,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);
                let subscription_client = client.subscription();
                let res = subscription_client
                    .get(UniverseId(universe_id), product, subscription, view)
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
