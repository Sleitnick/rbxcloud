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
    title: String,
    uri: String,
}

pub struct GetUniverseParams {
    pub api_key: String,
    pub universe_id: UniverseId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UniverseInfo {
    path: String,
    create_time: String,
    update_time: String,
    display_name: String,
    description: String,
    user: Option<String>,
    group: Option<String>,
    visibility: UniverseVisibility,
    facebook_social_link: UniverseSocialLink,
    twitter_social_link: UniverseSocialLink,
    youtube_social_link: UniverseSocialLink,
    twitch_social_link: UniverseSocialLink,
    discord_social_link: UniverseSocialLink,
    roblox_group_social_link: UniverseSocialLink,
    guilded_social_link: UniverseSocialLink,
    voice_chat_enabled: bool,
    age_rating: UniverseAgeRating,
    private_server_price_robux: u32,
    desktop_enabled: bool,
    mobile_enabled: bool,
    tablet_enabled: bool,
    console_enabled: bool,
    vr_enabled: bool,
}

pub struct UpdateUniverseParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub update_mask: String,
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

    let res = client
        .patch(url)
        .header("x-api-key", &params.api_key)
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

    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    Ok(())
}
