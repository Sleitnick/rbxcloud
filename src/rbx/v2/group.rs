use serde::{Deserialize, Serialize};

use crate::rbx::error::Error;

use super::http_err::handle_http_err;

#[derive(Debug, Clone, Copy)]
pub struct GroupId(pub u64);

impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct GetGroupParams {
    pub api_key: String,
    pub group_id: GroupId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupResponse {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub owner: String,
    pub member_count: u64,
    pub public_entry_allowed: bool,
    pub locked: bool,
    pub verified: bool,
}

pub struct GetGroupShoutParams {
    pub api_key: String,
    pub group_id: GroupId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupShoutResponse {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub content: String,
    pub poster: String,
}

pub async fn get_group(params: &GetGroupParams) -> Result<GetGroupResponse, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/groups/{groupId}",
        groupId = &params.group_id,
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

    let body = res.json::<GetGroupResponse>().await?;
    Ok(body)
}

pub async fn get_group_shout(params: &GetGroupShoutParams) -> Result<GetGroupShoutResponse, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/groups/{groupId}/shout",
        groupId = &params.group_id,
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

    let body = res.json::<GetGroupShoutResponse>().await?;
    Ok(body)
}
