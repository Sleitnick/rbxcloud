use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::UniverseId, util::QueryString};

use super::http_err::handle_http_err;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UniverseVisibility {
    VisibilityUnspecified,
    Public,
    Private,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UniverseAgeRating {
    AgeRatingUnspecified,
    AgeRatingAll,
    AgeRating9Plus,
    AgeRating13Plus,
    AgeRating17Plus,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UniverseSocialLink {
    pub title: String,
    pub uri: String,
}

pub struct GetUniverseParams {
    pub api_key: String,
    pub universe_id: UniverseId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UniverseInfo {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub display_name: String,
    pub description: String,
    pub user: Option<String>,
    pub group: Option<String>,
    pub visibility: UniverseVisibility,
    pub facebook_social_link: Option<UniverseSocialLink>,
    pub twitter_social_link: Option<UniverseSocialLink>,
    pub youtube_social_link: Option<UniverseSocialLink>,
    pub twitch_social_link: Option<UniverseSocialLink>,
    pub discord_social_link: Option<UniverseSocialLink>,
    pub roblox_group_social_link: Option<UniverseSocialLink>,
    pub guilded_social_link: Option<UniverseSocialLink>,
    pub voice_chat_enabled: bool,
    pub age_rating: UniverseAgeRating,
    pub private_server_price_robux: u32,
    pub desktop_enabled: bool,
    pub mobile_enabled: bool,
    pub tablet_enabled: bool,
    pub console_enabled: bool,
    pub vr_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUniverseInfo {
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
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<UniverseVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roblox_group_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guilded_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_rating: Option<UniverseAgeRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_price_robux: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tablet_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vr_enabled: Option<bool>,
}

pub struct UpdateUniverseParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub update_mask: String,
    pub info: UpdateUniverseInfo,
}

pub struct RestartUniverseServersParams {
    pub api_key: String,
    pub universe_id: UniverseId,
}

pub async fn get_universe(params: &GetUniverseParams) -> Result<UniverseInfo, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}",
        universeId = &params.universe_id,
    );

    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<UniverseInfo>().await?;
    Ok(body)
}

pub async fn update_universe(params: &UpdateUniverseParams) -> Result<UniverseInfo, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}",
        universeId = &params.universe_id,
    );

    let mut query: QueryString = vec![];
    query.push(("updateMask", params.update_mask.clone()));

    let body = serde_json::to_string(&params.info)?;
    println!("body: {}", body);

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

    let body = res.json::<UniverseInfo>().await?;
    Ok(body)
}

pub async fn restart_universe_servers(params: &RestartUniverseServersParams) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}:restartServers",
        universeId = &params.universe_id,
    );

    let body = "{}";

    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    Ok(())
}
