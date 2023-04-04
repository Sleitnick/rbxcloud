use clap::{Args, Subcommand};

use rbxcloud::rbx::{
    assets::{
        AssetCreation, AssetCreationContext, AssetCreator, AssetGroupCreator, AssetType,
        AssetUserCreator,
    },
    CreateAsset, GetAsset, RbxCloud, UpdateAsset,
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

    /// Update an asset
    Update {
        /// Asset type
        #[clap(short = 't', long, value_enum)]
        asset_type: AssetType,

        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// File content
        #[clap(short, long, value_parser)]
        file_content: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get asset information
    Get {
        /// Operation ID
        #[clap(short = 'i', long, value_parser)]
        operation_id: u64,

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

fn create_context_from_creator_type(
    creator_type: CreatorType,
    creator_id: u64,
    expected_price: Option<u64>,
) -> AssetCreationContext {
    match creator_type {
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
    }
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
                let creation_context =
                    create_context_from_creator_type(creator_type, creator_id, expected_price);
                let res = assets
                    .create(&CreateAsset {
                        asset: AssetCreation {
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

            AssetsCommands::Update {
                asset_type,
                asset_id,
                file_content,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let res = assets
                    .update(&UpdateAsset {
                        asset_id,
                        asset_type,
                        file_content,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::Get {
                operation_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let res = assets.get(&GetAsset { operation_id }).await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
