use std::fmt;

use anyhow::bail;
use md5::{Md5, Digest};
use reqwest::Response;
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreEntry {
	pub name: String,
	pub created_time: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListDataStoresResponse {
	pub datastores: Vec<DataStoreEntry>,
	pub next_page_cursor: Option<String>,
}

pub struct ListDataStoresParams {
    pub api_key: String,
    pub universe_id: u64,
	pub prefix: Option<String>,
	pub limit: u64,
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
		let details = self.error_details
			.iter()
			.map(|item| format!("{:?}", item.datastore_error_code))
			.collect::<Vec<String>>().join(", ");
        write!(f, "[{}] - {}", details, self.message)
    }
}

pub struct ListEntriesParams {
    pub api_key: String,
    pub universe_id: u64,
	pub datastore_name: String,
	pub scope: Option<String>,
	pub all_scopes: bool,
	pub prefix: Option<String>,
	pub limit: u64,
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
    pub universe_id: u64,
	pub datastore_name: String,
	pub scope: Option<String>,
	pub key: String,
}

pub struct SetEntryParams {
    pub api_key: String,
    pub universe_id: u64,
	pub datastore_name: String,
	pub scope: Option<String>,
	pub key: String,
	pub match_version: Option<String>,
	pub exclusive_create: Option<bool>,
	pub roblox_entry_user_ids: Option<Vec<u64>>,
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
    pub universe_id: u64,
	pub datastore_name: String,
	pub scope: Option<String>,
	pub key: String,
	pub roblox_entry_user_ids: Option<Vec<u64>>,
	pub roblox_entry_attributes: Option<String>,
	pub increment_by: f64,
}

pub struct DeleteEntryParams {
    pub api_key: String,
    pub universe_id: u64,
	pub datastore_name: String,
	pub scope: Option<String>,
	pub key: String,
}

pub struct ListEntryVersionsParams {
    pub api_key: String,
    pub universe_id: u64,
	pub datastore_name: String,
	pub scope: Option<String>,
	pub key: String,
	pub start_time: Option<String>,
	pub end_time: Option<String>,
	pub sort_order: String,
	pub limit: u64,
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
    pub universe_id: u64,
	pub datastore_name: String,
	pub scope: Option<String>,
	pub key: String,
	pub version_id: String,
}

async fn handle_res<T: DeserializeOwned>(res: Response) -> anyhow::Result<T> {
	match res.status().is_success() {
		true => {
			let body_res = res.json::<T>().await;
			match body_res {
				Ok(body) => Ok(body),
				Err(err) => bail!(err)
			}
		},
		false => {
			let err_res = res.json::<DataStoreErrorResponse>().await?;
			bail!(err_res)
		},
	}
}

async fn handle_res_string(res: Response) -> anyhow::Result<String> {
	match res.status().is_success() {
		true => {
			let body_res = res.text().await;
			match body_res {
				Ok(body) => Ok(body),
				Err(err) => bail!(err)
			}
		},
		false => {
			let err_res = res.json::<DataStoreErrorResponse>().await?;
			bail!(err_res)
		},
	}
}

async fn handle_res_ok(res: Response) -> anyhow::Result<()> {
	match res.status().is_success() {
		true => {
			Ok(())
		},
		false => {
			let err_res = res.json::<DataStoreErrorResponse>().await?;
			bail!(err_res)
		},
	}
}

pub async fn list_datastores(params: &ListDataStoresParams) -> anyhow::Result<ListDataStoresResponse> {
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/datastores/v1/universes/{universeId}/standard-datastores",
		universeId=params.universe_id
	);
	let mut query: Vec<(&str, String)> = vec![("limit", params.limit.to_string())];
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

pub async fn list_entries(params: &ListEntriesParams) -> anyhow::Result<ListEntriesResponse> {
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/datastores/v1/universes/{universeId}/standard-datastores/datastore/entries",
		universeId=params.universe_id
	);
	let mut query: Vec<(&str, String)> = vec![
		("datastoreName", params.datastore_name.clone()),
		("limit", params.limit.to_string()),
		("AllScopes", params.all_scopes.to_string()),
		("scope", params.scope.clone().unwrap_or("global".to_string())),
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

async fn get_entry_response(params: &GetEntryParams) -> anyhow::Result<Response> {
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/datastores/v1/universes/{universeId}/standard-datastores/datastore/entries/entry",
		universeId=params.universe_id
	);
	let query: Vec<(&str, String)> = vec![
		("datastoreName", params.datastore_name.clone()),
		("scope", params.scope.clone().unwrap_or("global".to_string())),
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

pub async fn get_entry_string(params: &GetEntryParams) -> anyhow::Result<String> {
	let res = get_entry_response(params).await?;
	handle_res_string(res).await
}

pub async fn get_entry<T: DeserializeOwned>(params: &GetEntryParams) -> anyhow::Result<T> {
	let res = get_entry_response(params).await?;
	handle_res::<T>(res).await
}

fn build_ids_csv(ids: &Option<Vec<u64>>) -> String {
	ids
		.as_ref()
		.unwrap_or(&vec![])
		.iter()
		.map(|id| format!("{}", id))
		.collect::<Vec<String>>()
		.join(",")
}

pub async fn set_entry(params: &SetEntryParams) -> anyhow::Result<SetEntryResponse> {
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/datastores/v1/universes/{universeId}/standard-datastores/datastore/entries/entry",
		universeId=params.universe_id
	);
	let mut query: Vec<(&str, String)> = vec![
		("datastoreName", params.datastore_name.clone()),
		("scope", params.scope.clone().unwrap_or("global".to_string())),
		("entryKey", params.key.clone()),
	];
	if let Some(match_version) = &params.match_version {
		query.push(("matchVersion", match_version.clone()));
	}
	if let Some(exclusive_create) = &params.exclusive_create {
		query.push(("exclusiveCreate", exclusive_create.to_string()));
	}
	let ids = build_ids_csv(&params.roblox_entry_user_ids);
	let mut hasher = Md5::new();
	hasher.update(&params.data.as_bytes());
	let checksum = hasher.finalize();
	let checksum_b64 = base64::encode(&checksum);
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
		.header("Content-Type", "application/json")
		.header("roblox-entry-userids", format!("[{}]", ids))
		.header("roblox-entry-attributes", params.roblox_entry_attributes.as_ref().unwrap_or(&"{}".to_string()))
		.header("content-md5", checksum_b64)
		.body(params.data.clone())
		.query(&query)
        .send()
        .await?;
	handle_res::<SetEntryResponse>(res).await
}

pub async fn increment_entry(params: &IncrementEntryParams) -> anyhow::Result<f64> {
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/datastores/v1/universes/{universeId}/standard-datastores/datastore/entries/entry/increment",
		universeId=params.universe_id
	);
	let query: Vec<(&str, String)> = vec![
		("datastoreName", params.datastore_name.clone()),
		("scope", params.scope.clone().unwrap_or("global".to_string())),
		("entryKey", params.key.clone()),
		("incrementBy", params.increment_by.to_string()),
	];
	let ids = build_ids_csv(&params.roblox_entry_user_ids);
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
		.header("roblox-entry-userids", format!("[{}]", ids))
		.header("roblox-entry-attributes", params.roblox_entry_attributes.as_ref().unwrap_or(&"{}".to_string()))
		.query(&query)
        .send()
        .await?;
	match handle_res_string(res).await {
		Ok(data) => {
			match data.parse::<f64>() {
				Ok(num) => {
					Ok(num)
				}
				Err(_) => {
					bail!(format!("failed to parse number from data: {}", data))
				}
			}
		}
		Err(err) => {
			bail!(err)
		}
	}
}

pub async fn delete_entry(params: &DeleteEntryParams) -> anyhow::Result<()> {
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/datastores/v1/universes/{universeId}/standard-datastores/datastore/entries/entry",
		universeId=params.universe_id
	);
	let query: Vec<(&str, String)> = vec![
		("datastoreName", params.datastore_name.clone()),
		("scope", params.scope.clone().unwrap_or("global".to_string())),
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

pub async fn list_entry_versions(params: &ListEntryVersionsParams) -> anyhow::Result<ListEntryVersionsResponse> {
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/datastores/v1/universes/{universeId}/standard-datastores/datastore/entries/entry/versions",
		universeId=params.universe_id
	);
	let mut query: Vec<(&str, String)> = vec![
		("datastoreName", params.datastore_name.clone()),
		("scope", params.scope.clone().unwrap_or("global".to_string())),
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

pub async fn get_entry_version(params: &GetEntryVersionParams) -> anyhow::Result<String> {
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/datastores/v1/universes/{universeId}/standard-datastores/datastore/entries/entry/versions/version",
		universeId=params.universe_id
	);
	let query: Vec<(&str, String)> = vec![
		("datastoreName", params.datastore_name.clone()),
		("scope", params.scope.clone().unwrap_or("global".to_string())),
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
