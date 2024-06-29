use super::http_err::handle_http_err;
use crate::rbx::{
    error::Error,
    types::{PlaceId, UniverseId},
    util::QueryString,
};
use serde::{Deserialize, Serialize};

pub struct GetPlaceParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceInfo {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub display_name: String,
    pub description: String,
    pub server_size: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePlaceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_size: Option<i32>,
}

pub struct UpdatePlaceParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub update_mask: String,
    pub info: UpdatePlaceInfo,
}

pub async fn get_place(params: &GetPlaceParams) -> Result<PlaceInfo, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}",
        universeId = &params.universe_id,
        placeId = &params.place_id,
    );

    let res = client
        .get(&url)
        .header("x-api-key", &params.api_key)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<PlaceInfo>().await?;
    Ok(body)
}

pub async fn update_place(params: &UpdatePlaceParams) -> Result<PlaceInfo, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}",
        universeId = &params.universe_id,
        placeId = &params.place_id,
    );

    let mut query: QueryString = vec![];
    query.push(("updateMask", params.update_mask.clone()));

    let body = serde_json::to_string(&params.info)?;

    let res = client
        .patch(url)
        .header("x-api-key", &params.api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .query(&query)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<PlaceInfo>().await?;
    Ok(body)
}
