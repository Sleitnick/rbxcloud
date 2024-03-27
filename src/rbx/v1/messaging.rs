//! Low-level Messaging API operations.
use serde_json::json;

use crate::rbx::error::Error;
use crate::rbx::v1::UniverseId;

/// Message publishing parameters.
pub struct PublishMessageParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub topic: String,
    pub message: String,
}

/// Publish a message.
pub async fn publish_message(params: &PublishMessageParams) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://apis.roblox.com/messaging-service/v1/universes/{universeId}/topics/{topic}",
        universeId = params.universe_id,
        topic = params.topic,
    );
    let body_json = json!({
        "message": &params.message,
    });
    let body = serde_json::to_string(&body_json)?;
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
        if code == 400 {
            return Err(Error::HttpStatusError {
                code,
                msg: "invalid request".to_string(),
            });
        } else if code == 401 {
            return Err(Error::HttpStatusError {
                code,
                msg: "api key not valid for operation".to_string(),
            });
        } else if code == 403 {
            return Err(Error::HttpStatusError {
                code,
                msg: "publish not allowed on place".to_string(),
            });
        } else if code == 500 {
            return Err(Error::HttpStatusError {
                code,
                msg: "internal server error".to_string(),
            });
        }
        return Err(Error::HttpStatusError {
            code,
            msg: status.canonical_reason().unwrap_or_default().to_string(),
        });
    }
    Ok(())
}
