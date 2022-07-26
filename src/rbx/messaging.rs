use anyhow::{bail, Context};
use serde_json::json;

use crate::rbx::error::RbxError;

pub struct PublishMessageParams {
	pub api_key: String,
	pub universe_id: u64,
	pub topic: String,
	pub message: String,
}

pub async fn publish_message(params: &PublishMessageParams) -> anyhow::Result<()> {
	// https://apis.roblox.com/messaging-service/v1/universes/{universeId}/topics/{topic}
	let client = reqwest::Client::new();
	let url = format!(
		"https://apis.roblox.com/messaging-service/v1/universes/{universeId}/topics/{topic}",
		universeId=params.universe_id,
		topic=params.topic,
	);
	let body_json = json!({
		// "message": serde_json::to_string(&serde_json::from_str::<Value>(&params.message).unwrap()).unwrap(),
		"message": &params.message,
	});
	let body = serde_json::to_string(&body_json).context("failed to parse json")?;
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
			bail!(RbxError::HttpStatusError { code: code, msg: "invalid request".to_string() });
        } else if code == 401 {
			bail!(RbxError::HttpStatusError { code: code, msg: "api key not valid for operation".to_string() });
        } else if code == 403 {
			bail!(RbxError::HttpStatusError { code: code, msg: "publish not allowed on place".to_string() });
        } else if code == 500 {
			bail!(RbxError::HttpStatusError { code: code, msg: "internal server error".to_string() });
        }
		bail!(RbxError::HttpStatusError { code: code, msg: status.canonical_reason().unwrap_or_default().to_string() });
    }
	Ok(())
}
