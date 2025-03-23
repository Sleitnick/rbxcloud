use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

use crate::rbx::{
    error::Error,
    types::{PlaceId, RobloxUserId, UniverseId},
    util::QueryString,
};

use super::http_err::handle_http_err;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameJoinRestriction {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    pub private_reason: String,
    pub display_reason: String,
    pub exclude_alt_accounts: bool,
    pub inherited: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRestriction {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<DateTime<Utc>>,
    pub user: String,
    pub game_join_restriction: GameJoinRestriction,
}

pub struct UpdateUserRestrictionParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: Option<PlaceId>,
    pub user_id: RobloxUserId,
    pub idempotency_key: Option<String>,
    pub active: Option<bool>,
    pub duration: Option<String>,
    pub private_reason: Option<String>,
    pub display_reason: Option<String>,
    pub exclude_alt_accounts: Option<bool>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UpdateUserRestriction {
    game_join_restriction: GameJoinRestriction,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRestrictionList {
    pub user_restrictions: Vec<UserRestriction>,
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameServerScript {
    // empty
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UserRestrictionModerator {
    RobloxUser(String),
    GameServerScript(GameServerScript),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRestrictionLog {
    pub user: String,
    pub place: String,
    pub create_time: String,
    pub active: bool,
    pub start_time: String,
    pub duration: String,
    pub private_reason: String,
    pub display_reason: String,
    pub exclude_alt_accounts: bool,
    pub moderator: UserRestrictionModerator,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRestrictionLogsList {
    pub logs: Vec<UserRestrictionLog>,
    pub next_page_token: Option<String>,
}

pub struct GetUserRestrictionParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: Option<PlaceId>,
    pub user_id: RobloxUserId,
}

pub struct ListUserRestrictionsParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: Option<PlaceId>,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub filter: Option<String>,
}

pub struct ListUserRestrictionLogsParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: Option<PlaceId>,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub filter: Option<String>,
}

pub async fn get_user_restriction(
    params: &GetUserRestrictionParams,
) -> Result<UserRestriction, Error> {
    let client = reqwest::Client::new();

    let url = if let Some(place_id) = params.place_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/user-restrictions/{user}",
			universeId = &params.universe_id,
			placeId = &place_id,
			user = &params.user_id,
		)
    } else {
        format!(
            "https://apis.roblox.com/cloud/v2/universes/{universeId}/user-restrictions/{user}",
            universeId = &params.universe_id,
            user = &params.user_id,
        )
    };

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

    let body = res.json::<UserRestriction>().await?;
    Ok(body)
}

pub async fn list_user_restrictions(
    params: &ListUserRestrictionsParams,
) -> Result<UserRestrictionList, Error> {
    let client = reqwest::Client::new();

    let url = if let Some(place_id) = params.place_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/user-restrictions",
			universeId = &params.universe_id,
			placeId = &place_id,
		)
    } else {
        format!(
            "https://apis.roblox.com/cloud/v2/universes/{universeId}/user-restrictions",
            universeId = &params.universe_id,
        )
    };

    let mut query: QueryString = vec![];
    if let Some(max_page_size) = &params.max_page_size {
        query.push(("maxPageSize", max_page_size.to_string()));
    }
    if let Some(page_token) = &params.page_token {
        query.push(("pageToken", page_token.to_string()));
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

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<UserRestrictionList>().await?;
    Ok(body)
}

pub async fn update_user_restriction(
    params: &UpdateUserRestrictionParams,
) -> Result<UserRestriction, Error> {
    let client = reqwest::Client::new();

    let url = if let Some(place_id) = params.place_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/user-restrictions/{user}",
			universeId = &params.universe_id,
			placeId = &place_id,
			user = &params.user_id,
		)
    } else {
        format!(
            "https://apis.roblox.com/cloud/v2/universes/{universeId}/user-restrictions/{user}",
            universeId = &params.universe_id,
            user = &params.user_id,
        )
    };

    let timestamp = Utc::now();

    let mut query: QueryString = vec![("updateMask", "gameJoinRestriction".to_string())];

    if let Some(idempotency_key) = &params.idempotency_key {
        query.push(("idempotencyKey.key", idempotency_key.to_string()));
        query.push((
            "idempotencyKey.firstSent",
            timestamp.to_rfc3339_opts(SecondsFormat::Millis, true),
        ));
    }

    let body = serde_json::to_string(&UpdateUserRestriction {
        game_join_restriction: GameJoinRestriction {
            active: params.active.unwrap_or(false),
            start_time: Some(timestamp),
            duration: params.duration.clone(),
            private_reason: params.private_reason.clone().unwrap_or("".into()),
            display_reason: params.display_reason.clone().unwrap_or("".into()),
            exclude_alt_accounts: params.exclude_alt_accounts.unwrap_or(false),
            inherited: false,
        },
    })?;

    let res = client
        .patch(url)
        .header("x-api-key", &params.api_key)
        .header("Content-Type", "application/json")
        .query(&query)
        .body(body)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<UserRestriction>().await?;
    Ok(body)
}

pub async fn list_user_restriction_logs(
    params: &ListUserRestrictionLogsParams,
) -> Result<UserRestrictionLogsList, Error> {
    let client = reqwest::Client::new();

    let url = if let Some(place_id) = params.place_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/user-restrictions:listLogs",
			universeId = &params.universe_id,
			placeId = &place_id,
		)
    } else {
        format!(
            "https://apis.roblox.com/cloud/v2/universes/{universeId}/user-restrictions:listLogs",
            universeId = &params.universe_id,
        )
    };

    let mut query: QueryString = vec![];
    if let Some(max_page_size) = &params.max_page_size {
        query.push(("maxPageSize", max_page_size.to_string()));
    }
    if let Some(page_token) = &params.page_token {
        query.push(("pageToken", page_token.to_string()));
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

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<UserRestrictionLogsList>().await?;
    Ok(body)
}
