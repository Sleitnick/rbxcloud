use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::RobloxUserId};

use super::http_err::handle_http_err;

#[derive(Deserialize, Serialize, Debug)]
pub enum NotificationType {
    TypeUnspecified,
    Moment,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::TypeUnspecified => "TYPE_UNSPECIFIED",
                Self::Moment => "MOMENT",
            }
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinExperience {
    pub launch_data: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub string_value: Option<String>,
    pub int64_value: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSource {
    pub universe: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPayload {
    pub message_id: String,
    #[serde(rename(serialize = "type"))]
    pub notification_type: NotificationType,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub join_experience: Option<JoinExperience>,
    pub analytics_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub source: NotificationSource,
    pub payload: NotificationPayload,
}

#[derive(Debug)]
pub struct NotificationParams {
    pub api_key: String,
    pub user_id: RobloxUserId,
    pub notification: Notification,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationResponse {
    pub path: String,
    pub id: String,
}

pub async fn send_notification(params: &NotificationParams) -> Result<NotificationResponse, Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://apis.roblox.com/cloud/v2/users/{user}/notifications",
        user = &params.user_id,
    );

    let body = serde_json::to_string(&params.notification)?;

    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .body(body)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<NotificationResponse>().await?;
    Ok(body)
}
