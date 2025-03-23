use clap::{Args, Subcommand};
use rbxcloud::rbx::{types::RobloxUserId, v2::Client};

#[derive(Debug, Subcommand)]
pub enum InventoryCommands {
    /// List inventory items for a given user
    List {
        /// Roblox user ID
        #[clap(short, long, value_parser)]
        user_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Max page size
        #[clap(short, long, value_parser)]
        max_page_size: Option<u32>,

        /// Next page token
        #[clap(short = 'n', long, value_parser)]
        page_token: Option<String>,

        /// Filter string
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Inventory {
    #[clap(subcommand)]
    command: InventoryCommands,
}

impl Inventory {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            InventoryCommands::List {
                user_id,
                pretty,
                max_page_size,
                page_token,
                filter,
                api_key,
            } => {
                let client = Client::new(&api_key);

                let inventory = client.inventory();

                let res = inventory
                    .list_inventory_items(RobloxUserId(user_id), max_page_size, page_token, filter)
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
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
