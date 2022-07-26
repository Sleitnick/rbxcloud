use anyhow::bail;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreEntry {
	name: String,
	created_time: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListDataStoresResponse {
	datastores: Vec<DataStoreEntry>,
	next_page_cursor: Option<String>,
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
struct DataStoreErrorResponse {
	error: String,
	message: String,
	error_details: Vec<DataStoreErrorDetail>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DataStoreErrorDetail {
	errorDetailType: String,
	datastoreErrorCode: DataStoreErrorCode,
}

#[derive(Deserialize, Debug)]
enum DataStoreErrorCode {
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
