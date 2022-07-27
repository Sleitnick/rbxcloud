use std::fmt;

use anyhow::bail;
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
	if res.status().is_success() {
		let body_res = res.json::<ListDataStoresResponse>().await;
		if let Err(e) = body_res {
			bail!(e);
		}
		Ok(body_res.unwrap())
	} else {
		let err_res = res.json::<DataStoreErrorResponse>().await?;
		Err(anyhow::anyhow!(format!("{:?}", err_res)))
	}
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
	if res.status().is_success() {
		let body_res = res.json::<ListEntriesResponse>().await;
		if let Err(e) = body_res {
			bail!(e);
		}
		Ok(body_res.unwrap())
	} else {
		let err_res = res.json::<DataStoreErrorResponse>().await?;
		Err(anyhow::anyhow!(format!("{}", err_res)))
	}
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
	if res.status().is_success() {
		let body_res = res.text().await;
		if let Err(e) = body_res {
			bail!(e);
		}
		Ok(body_res.unwrap())
	} else {
		let err_res = res.json::<DataStoreErrorResponse>().await?;
		Err(anyhow::anyhow!(format!("{}", err_res)))
	}
}

pub async fn get_entry<T: DeserializeOwned>(params: &GetEntryParams) -> anyhow::Result<T> {
	let res = get_entry_response(params).await?;
	if res.status().is_success() {
		let body_res = res.json::<T>().await;
		if let Err(e) = body_res {
			bail!(e);
		}
		Ok(body_res.unwrap())
	} else {
		let err_res = res.json::<DataStoreErrorResponse>().await?;
		Err(anyhow::anyhow!(format!("{}", err_res)))
	}
}
