use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::RobloxUserId, util::QueryString};

use super::http_err::handle_http_err;

pub struct GetUserParams {
    pub api_key: String,
    pub user_id: RobloxUserId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserSocialNetworkVisibility {
    SocialNetworkVisibilityUnspecified,
    NoOne,
    Friends,
    FriendsAndFollowing,
    FriendsFollowingAndFollowers,
    Everyone,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserSocialNetworkProfiles {
    facebook: String,
    twitter: String,
    youtube: String,
    twitch: String,
    guilded: String,
    visibility: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResponse {
    pub path: String,
    pub create_time: String,
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub about: String,
    pub locale: String,
    pub premium: Option<bool>,
    pub id_verified: Option<bool>,
    pub social_network_profiles: Option<UserSocialNetworkProfiles>,
}

pub struct GenerateUserThumbnailParams {
    pub api_key: String,
    pub user_id: RobloxUserId,
    pub size: Option<UserThumbnailSize>,
    pub format: Option<UserThumbnailFormat>,
    pub shape: Option<UserThumbnailShape>,
}

#[derive(Deserialize, Serialize, Debug, Clone, clap::ValueEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserThumbnailFormat {
    Png,
    Jpeg,
}

#[derive(Deserialize, Serialize, Debug, Clone, clap::ValueEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserThumbnailSize {
    Size48x48,
    Size50x50,
    Size60x60,
    Size75x75,
    Size100x100,
    Size110x110,
    Size150x150,
    Size180x180,
    Size352x352,
    Size420x420,
    Size720x720,
}

#[derive(Deserialize, Serialize, Debug, Clone, clap::ValueEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserThumbnailShape {
    Round,
    Square,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerateUserThumbnailOperationResponse {
    pub path: String,
    pub done: bool,
    pub response: GenerateUserThumbnailResponse,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerateUserThumbnailResponse {
    pub image_uri: String,
}

impl std::fmt::Display for UserThumbnailSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:?}",
            match self {
                Self::Size48x48 => "48",
                Self::Size50x50 => "50",
                Self::Size60x60 => "60",
                Self::Size75x75 => "75",
                Self::Size100x100 => "100",
                Self::Size110x110 => "110",
                Self::Size150x150 => "150",
                Self::Size180x180 => "180",
                Self::Size352x352 => "352",
                Self::Size420x420 => "420",
                Self::Size720x720 => "720",
            }
        )
    }
}

impl std::fmt::Display for UserThumbnailFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:?}",
            match self {
                Self::Png => "PNG",
                Self::Jpeg => "JPEG",
            }
        )
    }
}

impl std::fmt::Display for UserThumbnailShape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:?}",
            match self {
                Self::Round => "ROUND",
                Self::Square => "SQUARE",
            }
        )
    }
}

pub async fn get_user(params: &GetUserParams) -> Result<GetUserResponse, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/users/{user}",
        user = &params.user_id,
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

    let body = res.json::<GetUserResponse>().await?;
    Ok(body)
}

pub async fn generate_thumbnail(
    params: &GenerateUserThumbnailParams,
) -> Result<GenerateUserThumbnailOperationResponse, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/users/{user}:generateThumbnail",
        user = &params.user_id,
    );

    let mut query: QueryString = vec![];
    if let Some(size) = &params.size {
        query.push(("size", size.to_string()))
    }
    if let Some(format) = &params.format {
        query.push(("format", format.to_string()))
    }
    if let Some(shape) = &params.shape {
        query.push(("shape", shape.to_string()))
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

    let body = res.json::<GenerateUserThumbnailOperationResponse>().await?;
    Ok(body)
}
