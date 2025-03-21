use std::{fs, path::Path};

use crate::rbx::{error::Error, util::QueryString};
use reqwest::{multipart, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetUserCreator {
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetGroupCreator {
    pub group_id: String,
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
    pub expected_price: Option<u64>,
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
    pub filepath: String,
}

pub struct CreateAssetParamsWithContents<'a> {
    pub api_key: String,
    pub asset: AssetCreation,
    pub contents: &'a [u8],
}

pub struct UpdateAssetParams {
    pub api_key: String,
    pub asset_id: u64,
    pub asset_type: AssetType,
    pub filepath: String,
}

pub struct GetAssetOperationParams {
    pub api_key: String,
    pub operation_id: String,
}

pub struct GetAssetParams {
    pub api_key: String,
    pub asset_id: u64,
    pub read_mask: Option<String>,
}

pub struct ArchiveAssetParams {
    pub api_key: String,
    pub asset_id: u64,
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
pub struct AssetGetOperation {
    pub path: String,
    pub done: Option<bool>,
    pub response: Option<AssetGetOperationResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetGetOperationResponse {
    pub path: String,
    pub revision_id: String,
    pub revision_create_time: String,
    pub asset_id: String,
    pub display_name: String,
    pub description: String,
    pub asset_type: String,
    pub creation_context: AssetCreationContext,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    pub asset_type: AssetTypeCategory,
    pub asset_id: String,
    pub creation_context: AssetCreationContext,
    pub description: String,
    pub display_name: String,
    pub path: String,
    pub revision_id: String,
    pub revision_create_time: String,
    pub moderation_result: ModerationResult,
    pub state: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModerationResult {
    /// Note: There's a discrepancy between the Open Cloud docs and the actual
    /// data from the API regarding what this value can be, hence it has been
    /// left as a string and not an enum.
    pub moderation_state: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetErrorStatus {
    pub code: u64,
    pub message: String,
    pub details: Vec<ProtobufAny>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Deserialize)]
pub enum AssetType {
    AudioMp3,
    AudioOgg,
    AudioFlac,
    AudioWav,
    DecalPng,
    DecalJpeg,
    DecalBmp,
    DecalTga,
    ModelFbx,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Deserialize)]
pub enum AssetTypeCategory {
    Audio,
    Decal,
    Model,
}

impl AssetType {
    fn content_type(&self) -> &'static str {
        match self {
            Self::AudioMp3 => "audio/mpeg",
            Self::AudioOgg => "audio/ogg",
            Self::AudioFlac => "audio/flac",
            Self::AudioWav => "audio/wav",
            Self::DecalPng => "image/png",
            Self::DecalJpeg => "image/jpeg",
            Self::DecalBmp => "image/bmp",
            Self::DecalTga => "image/tga",
            Self::ModelFbx => "model/fbx",
        }
    }

    fn asset_type(&self) -> &'static str {
        match self {
            Self::AudioMp3 | Self::AudioOgg | Self::AudioFlac | Self::AudioWav => "Audio",
            Self::DecalPng | Self::DecalJpeg | Self::DecalBmp | Self::DecalTga => "Decal",
            Self::ModelFbx => "Model",
        }
    }

    pub fn try_from_extension(extension: &str) -> Result<Self, crate::rbx::error::Error> {
        match extension.to_lowercase().as_str() {
            "mp3" => Ok(Self::AudioMp3),
            "ogg" => Ok(Self::AudioOgg),
            "flac" => Ok(Self::AudioFlac),
            "wav" => Ok(Self::AudioWav),
            "png" => Ok(Self::DecalPng),
            "jpg" => Ok(Self::DecalJpeg),
            "jpeg" => Ok(Self::DecalJpeg),
            "bmp" => Ok(Self::DecalBmp),
            "tga" => Ok(Self::DecalTga),
            "fbx" => Ok(Self::ModelFbx),
            _ => Err(crate::rbx::error::Error::InferAssetTypeError(
                "Unknown extension".to_string(),
            )),
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
    let file_name = Path::new(&params.filepath)
        .file_name()
        .ok_or_else(|| Error::FileLoadError("Failed to parse file name from file path".into()))?
        .to_os_string()
        .into_string()
        .map_err(|err| {
            Error::FileLoadError(format!("Failed to convert file name into string: {err:?}"))
        })?;

    let asset_info = serde_json::to_string(&params.asset)?;
    let file = multipart::Part::bytes(fs::read(&params.filepath)?)
        .file_name(file_name)
        .mime_str(params.asset.asset_type.content_type())?;

    let form = multipart::Form::new()
        .text("request", asset_info)
        .part("fileContent", file);

    let client = reqwest::Client::new();
    let url = build_url(None);
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .multipart(form)
        .send()
        .await?;
    handle_res::<AssetOperation>(res).await
}

pub async fn create_asset_with_contents<'a>(
    params: &CreateAssetParamsWithContents<'a>,
) -> Result<AssetOperation, Error> {
    let file = multipart::Part::bytes(params.contents.to_vec())
        .file_name(params.asset.display_name.clone())
        .mime_str(params.asset.asset_type.content_type())?;
    let asset_info = serde_json::to_string(&params.asset)?;

    let form = multipart::Form::new()
        .text("request", asset_info)
        .part("fileContent", file);

    let client = reqwest::Client::new();
    let url = build_url(None);
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .multipart(form)
        .send()
        .await?;
    handle_res::<AssetOperation>(res).await
}

pub async fn update_asset(params: &UpdateAssetParams) -> Result<AssetOperation, Error> {
    let file_name = Path::new(&params.filepath)
        .file_name()
        .ok_or_else(|| Error::FileLoadError("Failed to parse file name from file path".into()))?
        .to_os_string()
        .into_string()
        .map_err(|err| {
            Error::FileLoadError(format!("Failed to convert file name into string: {err:?}"))
        })?;

    let file = multipart::Part::bytes(fs::read(&params.filepath)?)
        .file_name(file_name)
        .mime_str(params.asset_type.content_type())?;

    let request_json = json!({
        "assetId": params.asset_id,
    });
    let request = serde_json::to_string(&request_json)?;

    let form = multipart::Form::new()
        .text("request", request)
        .part("fileContent", file);

    let client = reqwest::Client::new();
    let url = build_url(Some(params.asset_id));
    let res = client
        .patch(url)
        .header("x-api-key", &params.api_key)
        .multipart(form)
        .send()
        .await?;
    handle_res::<AssetOperation>(res).await
}

pub async fn get_operation(params: &GetAssetOperationParams) -> Result<AssetGetOperation, Error> {
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
    handle_res::<AssetGetOperation>(res).await
}

pub async fn get_asset(params: &GetAssetParams) -> Result<AssetInfo, Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://apis.roblox.com/assets/v1/assets/{assetId}",
        assetId = params.asset_id
    );
    let mut query: QueryString = vec![];
    if let Some(read_mask) = &params.read_mask {
        query.push(("readMask", read_mask.clone()));
    }
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    handle_res::<AssetInfo>(res).await
}

pub async fn archive_asset(params: &ArchiveAssetParams) -> Result<AssetInfo, Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://apis.roblox.com/assets/v1/assets/{assetId}:archive",
        assetId = params.asset_id
    );
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    handle_res::<AssetInfo>(res).await
}

pub async fn restore_asset(params: &ArchiveAssetParams) -> Result<AssetInfo, Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://apis.roblox.com/assets/v1/assets/{assetId}:restore",
        assetId = params.asset_id
    );
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    handle_res::<AssetInfo>(res).await
}
