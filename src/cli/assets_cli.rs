use std::path::Path;

use clap::{Args, Subcommand};

use rbxcloud::rbx::{
    error::Error,
    v1::{
        assets::{
            AssetCreation, AssetCreationContext, AssetCreator, AssetGroupCreator, AssetType,
            AssetUserCreator,
        },
        ArchiveAsset, CreateAsset, GetAsset, GetAssetOperation, RbxCloud, UpdateAsset,
    },
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
        asset_type: Option<AssetType>,

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

        /// File (full or relative path)
        #[clap(short, long, value_parser)]
        filepath: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update an asset
    Update {
        /// Asset type
        #[clap(short = 't', long, value_enum)]
        asset_type: Option<AssetType>,

        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// File (full or relative path)
        #[clap(short, long, value_parser)]
        filepath: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get asset operation information
    GetOperation {
        /// Operation ID
        #[clap(short = 'i', long, value_parser)]
        operation_id: String,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get asset information
    Get {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        #[clap(short = 'm', long, value_parser)]
        read_mask: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Archive an asset
    Archive {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Restore an archived asset
    Restore {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

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
    let expected_price = expected_price.unwrap_or(0);
    match creator_type {
        CreatorType::User => AssetCreationContext {
            expected_price: Some(expected_price),
            creator: AssetCreator::User(AssetUserCreator {
                user_id: creator_id.to_string(),
            }),
        },
        CreatorType::Group => AssetCreationContext {
            expected_price: Some(expected_price),
            creator: AssetCreator::Group(AssetGroupCreator {
                group_id: creator_id.to_string(),
            }),
        },
    }
}

fn infer_asset_type_from_filepath(filepath: &String) -> Result<AssetType, Error> {
    let path = Path::new(filepath);
    match path.extension() {
        Some(ext) => {
            let ext = ext
                .to_str()
                .ok_or_else(|| Error::InferAssetTypeError(filepath.into()))?;
            AssetType::try_from_extension(ext)
        }
        None => Err(Error::InferAssetTypeError(filepath.into())),
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
                filepath,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let creation_context =
                    create_context_from_creator_type(creator_type, creator_id, expected_price);
                let asset_type = match asset_type {
                    Some(t) => Ok(t),
                    None => infer_asset_type_from_filepath(&filepath),
                }?;
                let res = assets
                    .create(&CreateAsset {
                        asset: AssetCreation {
                            asset_type,
                            display_name,
                            description,
                            creation_context,
                        },
                        filepath: filepath.clone(),
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
                filepath,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let asset_type = match asset_type {
                    Some(t) => Ok(t),
                    None => infer_asset_type_from_filepath(&filepath),
                }?;
                let res = assets
                    .update(&UpdateAsset {
                        asset_id,
                        asset_type,
                        filepath,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::GetOperation {
                operation_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let res = assets
                    .get_operation(&GetAssetOperation { operation_id })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::Get {
                asset_id,
                read_mask,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let res = assets
                    .get(&GetAsset {
                        asset_id,
                        read_mask,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::Archive { asset_id, api_key } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let res = assets.archive(&ArchiveAsset { asset_id }).await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::Restore { asset_id, api_key } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let assets = rbx_cloud.assets();
                let res = assets.restore(&ArchiveAsset { asset_id }).await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
