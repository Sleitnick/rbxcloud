//! Low-level OrderedDataStore API operations.
//!
//! Typically, these operations should be consumed through the `RbxExperience`
//! struct, obtained through the `RbxCloud` struct.
//!

use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize};

use crate::rbx::{
    ds_error::DataStoreErrorResponse, error::Error, util::QueryString, PageSize, UniverseId,
};

pub struct OrderedListEntriesParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub max_page_size: Option<PageSize>,
    pub page_token: Option<String>,
    pub order_by: Option<String>,
    pub filter: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OrderedEntry {
    pub path: String,
    pub id: String,
    pub value: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderedListEntriesResponse {
    pub entries: Vec<OrderedEntry>,
    pub next_page_token: Option<String>,
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

fn build_url(endpoint: &str, universe_id: UniverseId, scope: Option<&str>) -> String {
    let s = scope.unwrap_or("global");
    if endpoint.is_empty() {
        format!("https://apis.roblox.com/ordered-data-stores/v1/universes/{universe_id}/orderedDataStores/scopes/{s}",)
    } else {
        format!(
			"https://apis.roblox.com/ordered-data-stores/v1/universes/{universe_id}/orderedDataStores/scopes/{s}{endpoint}",
		)
    }
}

/// List all entries of a DataStore.
pub async fn list_entries(
    params: &OrderedListEntriesParams,
) -> Result<OrderedListEntriesResponse, Error> {
    let client = reqwest::Client::new();
    let url = build_url("/entries", params.universe_id, params.scope.as_deref());
    let mut query: QueryString = vec![];
    if let Some(max_page_size) = &params.max_page_size {
        query.push(("max_page_size", max_page_size.to_string()));
    }
    if let Some(page_token) = &params.page_token {
        query.push(("page_token", page_token.to_string()));
    }
    if let Some(order_by) = &params.order_by {
        query.push(("order_by", order_by.to_string()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.to_string()));
    }
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    handle_res::<OrderedListEntriesResponse>(res).await
}
