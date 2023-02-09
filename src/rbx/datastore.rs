//! Low-level DataStore API operations.
//!
//! Typically, these operations should be consumed through the `RbxExperience`
//! struct, obtained through the `RbxCloud` struct.

use std::fmt;

use md5::{Digest, Md5};
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize};

use crate::rbx::{error::Error, ReturnLimit, RobloxUserId, UniverseId};

type QueryString = Vec<(&'static str, String)>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListDataStoreEntry {
    pub name: String,
    pub created_time: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListDataStoresResponse {
    pub datastores: Vec<ListDataStoreEntry>,
    pub next_page_cursor: Option<String>,
}

pub struct ListDataStoresParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub prefix: Option<String>,
    pub limit: ReturnLimit,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreErrorResponse {
    pub error: String,
    pub message: String,
    pub error_details: Vec<DataStoreErrorDetail>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreErrorDetail {
    pub error_detail_type: String,
    pub datastore_error_code: DataStoreErrorCode,
}

#[derive(Deserialize, Debug)]
pub enum DataStoreErrorCode {
    ContentLengthRequired,
    InvalidUniverseId,
    InvalidCursor,
    InvalidVersionId,
    ExistingValueNotNumeric,
    IncrementValueTooLarge,
    IncrementValueTooSmall,
    InvalidDataStoreScope,
    InvalidEntryKey,
    InvalidDataStoreName,
    InvalidStartTime,
    InvalidEndTime,
    InvalidAttributes,
    InvalidUserIds,
    ExclusiveCreateAndMatchVersionCannotBeSet,
    ContentTooBig,
    ChecksumMismatch,
    ContentNotJson,
    InvalidSortOrder,
    Forbidden,
    InsufficientScope,
    DatastoreNotFound,
    EntryNotFound,
    VersionNotFound,
    TooManyRequests,
    Unknown,
}

impl fmt::Display for DataStoreErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let details = self
            .error_details
            .iter()
            .map(|item| format!("{:?}", item.datastore_error_code))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}] - {}", details, self.message)
    }
}

pub struct ListEntriesParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub datastore_name: String,
    pub scope: Option<String>,
    pub all_scopes: bool,
    pub prefix: Option<String>,
    pub limit: ReturnLimit,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListEntriesResponse {
    pub keys: Vec<ListEntriesKey>,
    pub next_page_cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListEntriesKey {
    pub scope: String,
    pub key: String,
}

pub struct GetEntryParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub datastore_name: String,
    pub scope: Option<String>,
    pub key: String,
}

pub struct SetEntryParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub datastore_name: String,
    pub scope: Option<String>,
    pub key: String,
    pub match_version: Option<String>,
    pub exclusive_create: Option<bool>,
    pub roblox_entry_user_ids: Option<Vec<RobloxUserId>>,
    pub roblox_entry_attributes: Option<String>,
    pub data: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetEntryResponse {
    pub version: String,
    pub deleted: bool,
    pub content_length: u64,
    pub created_time: String,
    pub object_created_time: String,
}

pub struct IncrementEntryParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub datastore_name: String,
    pub scope: Option<String>,
    pub key: String,
    pub roblox_entry_user_ids: Option<Vec<RobloxUserId>>,
    pub roblox_entry_attributes: Option<String>,
    pub increment_by: f64,
}

pub struct DeleteEntryParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub datastore_name: String,
    pub scope: Option<String>,
    pub key: String,
}

pub struct ListEntryVersionsParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub datastore_name: String,
    pub scope: Option<String>,
    pub key: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub sort_order: String,
    pub limit: ReturnLimit,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListEntryVersionsResponse {
    pub versions: Vec<ListEntryVersion>,
    pub next_page_cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListEntryVersion {
    pub version: String,
    pub deleted: bool,
    pub content_length: u64,
    pub created_time: String,
    pub object_created_time: String,
}

pub struct GetEntryVersionParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub datastore_name: String,
    pub scope: Option<String>,
    pub key: String,
    pub version_id: String,
}

async fn handle_res<T: DeserializeOwned>(res: Response) -> Result<T, Error> {
    match res.status().is_success() {
        true => {
            let body = res.json::<T>().await?;
            Ok(body)
        }
        false => {
            let err_res = res.json::<DataStoreErrorResponse>().await?;
            Err(Error::DataStoreError(err_res))
        }
    }
}

async fn handle_res_string(res: Response) -> Result<String, Error> {
    match res.status().is_success() {
        true => {
            let body = res.text().await?;
            Ok(body)
        }
        false => {
            let err_res = res.json::<DataStoreErrorResponse>().await?;
            Err(Error::DataStoreError(err_res))
        }
    }
}

async fn handle_res_ok(res: Response) -> Result<(), Error> {
    match res.status().is_success() {
        true => Ok(()),
        false => {
            let err_res = res.json::<DataStoreErrorResponse>().await?;
            Err(Error::DataStoreError(err_res))
        }
    }
}

fn build_url(endpoint: &str, universe_id: UniverseId) -> String {
    if endpoint.is_empty() {
        format!("https://apis.roblox.com/datastores/v1/universes/{universe_id}/standard-datastores",)
    } else {
        format!(
			"https://apis.roblox.com/datastores/v1/universes/{universe_id}/standard-datastores{endpoint}",
		)
    }
}

#[inline]
fn get_checksum_base64(data: &String) -> String {
    let mut md5_hash = Md5::new();
    md5_hash.update(data.as_bytes());
    base64::encode(md5_hash.finalize())
}

/// List all DataStores within an experience.
pub async fn list_datastores(
    params: &ListDataStoresParams,
) -> Result<ListDataStoresResponse, Error> {
    let client = reqwest::Client::new();
    let url = build_url("", params.universe_id);
    let mut query: QueryString = vec![("limit", params.limit.to_string())];
    if let Some(prefix) = &params.prefix {
        query.push(("prefix", prefix.clone()));
    }
    if let Some(cursor) = &params.cursor {
        query.push(("cursor", cursor.clone()));
    }
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    handle_res::<ListDataStoresResponse>(res).await
}

/// List all entries of a DataStore.
pub async fn list_entries(params: &ListEntriesParams) -> Result<ListEntriesResponse, Error> {
    let client = reqwest::Client::new();
    let url = build_url("/datastore/entries", params.universe_id);
    let mut query: QueryString = vec![
        ("datastoreName", params.datastore_name.clone()),
        ("limit", params.limit.to_string()),
        ("AllScopes", params.all_scopes.to_string()),
        (
            "scope",
            params.scope.clone().unwrap_or_else(|| "global".to_string()),
        ),
    ];
    if let Some(prefix) = &params.prefix {
        query.push(("prefix", prefix.clone()));
    }
    if let Some(cursor) = &params.cursor {
        query.push(("cursor", cursor.clone()));
    }
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    handle_res::<ListEntriesResponse>(res).await
}

async fn get_entry_response(params: &GetEntryParams) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let url = build_url("/datastore/entries/entry", params.universe_id);
    let query: QueryString = vec![
        ("datastoreName", params.datastore_name.clone()),
        (
            "scope",
            params.scope.clone().unwrap_or_else(|| "global".to_string()),
        ),
        ("entryKey", params.key.clone()),
    ];
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    Ok(res)
}

/// Get the value of an entry as a string.
pub async fn get_entry_string(params: &GetEntryParams) -> Result<String, Error> {
    let res = get_entry_response(params).await?;
    handle_res_string(res).await
}

/// Get the value of an entry as a JSON-deserialized type `T`.
pub async fn get_entry<T: DeserializeOwned>(params: &GetEntryParams) -> Result<T, Error> {
    let res = get_entry_response(params).await?;
    handle_res::<T>(res).await
}

fn build_ids_csv(ids: &Option<Vec<RobloxUserId>>) -> String {
    ids.as_ref()
        .unwrap_or(&vec![])
        .iter()
        .map(|id| format!("{id}"))
        .collect::<Vec<String>>()
        .join(",")
}

/// Set the value of an entry.
pub async fn set_entry(params: &SetEntryParams) -> Result<SetEntryResponse, Error> {
    let client = reqwest::Client::new();
    let url = build_url("/datastore/entries/entry", params.universe_id);
    let mut query: QueryString = vec![
        ("datastoreName", params.datastore_name.clone()),
        (
            "scope",
            params.scope.clone().unwrap_or_else(|| "global".to_string()),
        ),
        ("entryKey", params.key.clone()),
    ];
    if let Some(match_version) = &params.match_version {
        query.push(("matchVersion", match_version.clone()));
    }
    if let Some(exclusive_create) = &params.exclusive_create {
        query.push(("exclusiveCreate", exclusive_create.to_string()));
    }
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .header("Content-Type", "application/json")
        .header(
            "roblox-entry-userids",
            format!("[{}]", build_ids_csv(&params.roblox_entry_user_ids)),
        )
        .header(
            "roblox-entry-attributes",
            params
                .roblox_entry_attributes
                .as_ref()
                .unwrap_or(&String::from("{}")),
        )
        .header("content-md5", get_checksum_base64(&params.data))
        .body(params.data.clone())
        .query(&query)
        .send()
        .await?;
    handle_res::<SetEntryResponse>(res).await
}

/// Increment the value of an entry.
pub async fn increment_entry(params: &IncrementEntryParams) -> Result<f64, Error> {
    let client = reqwest::Client::new();
    let url = build_url("/datastore/entries/entry/increment", params.universe_id);
    let query: QueryString = vec![
        ("datastoreName", params.datastore_name.clone()),
        (
            "scope",
            params.scope.clone().unwrap_or_else(|| "global".to_string()),
        ),
        ("entryKey", params.key.clone()),
        ("incrementBy", params.increment_by.to_string()),
    ];
    let ids = build_ids_csv(&params.roblox_entry_user_ids);
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .header("roblox-entry-userids", format!("[{ids}]"))
        .header(
            "roblox-entry-attributes",
            params
                .roblox_entry_attributes
                .as_ref()
                .unwrap_or(&"{}".to_string()),
        )
        .query(&query)
        .send()
        .await?;
    match handle_res_string(res).await {
        Ok(data) => match data.parse::<f64>() {
            Ok(num) => Ok(num),
            Err(e) => Err(e.into()),
        },
        Err(err) => Err(err),
    }
}

/// Delete an entry.
pub async fn delete_entry(params: &DeleteEntryParams) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = build_url("/datastore/entries/entry", params.universe_id);
    let query: QueryString = vec![
        ("datastoreName", params.datastore_name.clone()),
        (
            "scope",
            params.scope.clone().unwrap_or_else(|| "global".to_string()),
        ),
        ("entryKey", params.key.clone()),
    ];
    let res = client
        .delete(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    handle_res_ok(res).await
}

/// List all of the versions of an entry.
pub async fn list_entry_versions(
    params: &ListEntryVersionsParams,
) -> Result<ListEntryVersionsResponse, Error> {
    let client = reqwest::Client::new();
    let url = build_url("/datastore/entries/entry/versions", params.universe_id);
    let mut query: QueryString = vec![
        ("datastoreName", params.datastore_name.clone()),
        (
            "scope",
            params.scope.clone().unwrap_or_else(|| "global".to_string()),
        ),
        ("entryKey", params.key.to_string()),
        ("limit", params.limit.to_string()),
        ("sortOrder", params.sort_order.to_string()),
    ];
    if let Some(start_time) = &params.start_time {
        query.push(("startTime", start_time.clone()));
    }
    if let Some(end_time) = &params.end_time {
        query.push(("endTime", end_time.clone()));
    }
    if let Some(cursor) = &params.cursor {
        query.push(("cursor", cursor.clone()));
    }
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    handle_res::<ListEntryVersionsResponse>(res).await
}

/// Get the value of a specific entry version.
pub async fn get_entry_version(params: &GetEntryVersionParams) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let url = build_url(
        "/datastore/entries/entry/versions/version",
        params.universe_id,
    );
    let query: QueryString = vec![
        ("datastoreName", params.datastore_name.clone()),
        (
            "scope",
            params.scope.clone().unwrap_or_else(|| "global".to_string()),
        ),
        ("entryKey", params.key.to_string()),
        ("versionId", params.version_id.to_string()),
    ];
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    handle_res_string(res).await
}
