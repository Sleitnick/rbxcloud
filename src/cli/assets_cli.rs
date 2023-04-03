use clap::{Args, Subcommand};

use rbxcloud::rbx::{
    assets::{
        AssetCreationContext, AssetCreator, AssetGroupCreator, AssetInfo, AssetType,
        AssetUserCreator,
    },
    CreateAsset, RbxCloud,
};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum CreatorType {
    User,
    Group,
}

#[derive(Debug, Subcommand)]
pub enum AssetsCommands {
    /// Create an asset
    Create {
        /// Asset type
        #[clap(short = 't', long, value_enum)]
        asset_type: AssetType,

        /// Display name
        #[clap(short = 'n', long, value_parser)]
        display_name: String,

        /// Description
        #[clap(short, long, value_parser)]
        description: String,

        /// Expected Robux price
        #[clap(short, long, value_parser)]
        expected_price: Option<u64>,

        /// Creator ID
        #[clap(short = 'i', long, value_parser)]
        creator_id: u64,

        /// Creator type
        #[clap(short, long, value_enum)]
        creator_type: CreatorType,

        /// File content
        #[clap(short, long, value_parser)]
        file_content: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Assets {
    #[clap(subcommand)]
    command: AssetsCommands,
}

impl Assets {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            AssetsCommands::Create {
                asset_type,
                display_name,
                description,
                expected_price,
                creator_id,
                creator_type,
                file_content,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let creation_context = match creator_type {
                    CreatorType::User => AssetCreationContext {
                        expected_price: expected_price.unwrap_or(0),
                        creator: AssetCreator::User(AssetUserCreator {
                            user_id: creator_id,
                        }),
                    },
                    CreatorType::Group => AssetCreationContext {
                        expected_price: expected_price.unwrap_or(0),
                        creator: AssetCreator::Group(AssetGroupCreator {
                            group_id: creator_id,
                        }),
                    },
                };
                let res = assets
                    .create(&CreateAsset {
                        asset: AssetInfo {
                            asset_type,
                            display_name,
                            description,
                            creation_context,
                        },
                        file_content,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
