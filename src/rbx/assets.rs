use crate::rbx::error::Error;
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

use super::util::QueryString;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetUserCreator {
    pub user_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetGroupCreator {
    pub group_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AssetCreator {
    User(AssetUserCreator),
    Group(AssetGroupCreator),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetCreationContext {
    pub creator: AssetCreator,
    pub expected_price: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_type: String,
    pub asset_id: u64,
    pub creation_context: AssetCreationContext,
    pub description: String,
    pub display_name: String,
    pub path: String,
    pub revision_id: String,
    pub revision_create_time: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetCreation {
    pub asset_type: AssetType,
    pub display_name: String,
    pub description: String,
    pub creation_context: AssetCreationContext,
}

pub struct CreateAssetParams {
    pub api_key: String,
    pub asset: AssetCreation,
    pub file_content: String,
}

pub struct UpdateAssetParams {
    pub api_key: String,
    pub asset_id: u64,
    pub asset_type: AssetType,
    pub file_content: String,
}

pub struct GetAssetParams {
    pub api_key: String,
    pub operation_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetOperation {
    pub path: Option<String>,
    pub metadata: Option<ProtobufAny>,
    pub done: Option<bool>,
    pub error: Option<AssetErrorStatus>,
    pub response: Option<ProtobufAny>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProtobufAny {
    #[serde(rename = "@type")]
    pub message_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetErrorStatus {
    pub code: u64,
    pub message: String,
    pub details: Vec<ProtobufAny>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum AssetType {
    AudioMp3,
    AudioOgg,
    DecalPng,
    DecalJpeg,
    DecalBmp,
    DecalTga,
    ModelFbx,
}

impl AssetType {
    fn content_type(&self) -> &'static str {
        match self {
            Self::AudioMp3 => "audio/mpeg",
            Self::AudioOgg => "audio/ogg",
            Self::DecalPng => "image/png",
            Self::DecalJpeg => "image/jpeg",
            Self::DecalBmp => "image/bmp",
            Self::DecalTga => "image/tga",
            Self::ModelFbx => "model/fbx",
        }
    }

    fn asset_type(&self) -> &'static str {
        match self {
            Self::AudioMp3 | Self::AudioOgg => "Audio",
            Self::DecalPng | Self::DecalJpeg | Self::DecalBmp | Self::DecalTga => "Decal",
            Self::ModelFbx => "Model",
        }
    }
}

impl Serialize for AssetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.asset_type())
    }
}

async fn handle_res<T: DeserializeOwned>(res: Response) -> Result<T, Error> {
    let status = res.status();
    match status.is_success() {
        true => {
            let body = res.json::<T>().await?;
            Ok(body)
        }
        false => {
            let text = res.text().await?;
            Err(Error::HttpStatusError {
                code: status.as_u16(),
                msg: text,
            })
        }
    }
}

fn build_url(asset_id: Option<u64>) -> String {
    if let Some(asset_id) = asset_id {
        format!("https://apis.roblox.com/assets/v1/assets/{asset_id}")
    } else {
        "https://apis.roblox.com/assets/v1/assets".to_string()
    }
}

pub async fn create_asset(params: &CreateAssetParams) -> Result<AssetOperation, Error> {
    let client = reqwest::Client::new();
    let url = build_url(None);
    let asset_info = serde_json::to_string(&params.asset)?;
    let form_params: QueryString = vec![
        ("fileContents", params.file_content.to_string()),
        ("request", asset_info),
        ("type", params.asset.asset_type.content_type().to_string()),
    ];
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .form(&form_params)
        .send()
        .await?;
    handle_res::<AssetOperation>(res).await
}

pub async fn update_asset(params: &UpdateAssetParams) -> Result<AssetOperation, Error> {
    let client = reqwest::Client::new();
    let url = build_url(Some(params.asset_id));
    let request_json = json!({
        "assetId": params.asset_id,
    });
    let request = serde_json::to_string(&request_json)?;
    let form_params: QueryString = vec![
        ("request", request),
        ("fileContents", params.file_content.to_string()),
        ("type", params.asset_type.content_type().to_string()),
    ];
    let res = client
        .patch(url)
        .header("x-api-key", &params.api_key)
        .form(&form_params)
        .send()
        .await?;
    handle_res::<AssetOperation>(res).await
}

pub async fn get_asset(params: &GetAssetParams) -> Result<Asset, Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://apis.roblox.com/assets/v1/operations/{operationId}",
        operationId = params.operation_id
    );
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .send()
        .await?;
    handle_res::<Asset>(res).await
}
