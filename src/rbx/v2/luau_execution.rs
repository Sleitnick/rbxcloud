use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::rbx::{
    error::Error,
    types::{PlaceId, UniverseId},
};

use super::http_err::handle_http_err;

#[derive(Debug)]
pub struct CreateLuauExecutionTaskParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_id: Option<String>,
    pub script: String,
    pub timeout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewLuauExecutionSessionTask {
    pub path: String,
    pub user: String,
    pub state: LuauExecutionState,
    pub script: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLuauExecutionSessionTaskParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_id: Option<String>,
    pub session_id: String,
    pub task_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLuauExecutionSessionTaskLogsParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_id: Option<String>,
    pub session_id: String,
    pub task_id: String,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub view: LuauExecutionTaskLogView,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuauExecutionTaskLogView {
    Flat,
    Structured,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionSessionTaskLogPage {
    pub luau_execution_session_task_logs: Vec<LuauExecutionSessionTaskLog>,
    pub next_page_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionSessionTaskLog {
    pub path: String,
    pub messages: Vec<String>,
    pub structured_messages: Vec<StructuredMessage>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructuredMessage {
    pub message: String,
    pub create_time: String,
    pub message_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionSessionTask {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub user: String,
    pub state: LuauExecutionState,
    pub output: LuauExecutionOutput,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionOutput {
    pub results: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuauExecutionState {
    StateUnspecified,
    Queued,
    Processing,
    Cancelled,
    Complete,
    Failed,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LuauExecutionInput {
    pub script: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

pub async fn create_luau_execution_task(
    params: &CreateLuauExecutionTaskParams,
) -> Result<NewLuauExecutionSessionTask, Error> {
    let client = reqwest::Client::new();

    let url = if let Some(version_id) = &params.version_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/versions/{versionId}/luau-execution-session-tasks",
			universeId = &params.universe_id,
			placeId = &params.place_id,
			versionId = version_id,
		)
    } else {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/luau-execution-session-tasks",
			universeId = &params.universe_id,
			placeId = &params.place_id,
		)
    };

    let input = LuauExecutionInput {
        script: params.script.clone(),
        timeout: params.timeout.clone(),
    };

    let req_body = serde_json::to_string(&input)?;

    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .header("Content-Type", "application/json")
        .body(req_body)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success() {
        let code = status.as_u16();
        return handle_http_err(code);
    }

    let body = res.json::<NewLuauExecutionSessionTask>().await?;
    Ok(body)
}

pub async fn get_luau_execution_task(
    params: &GetLuauExecutionSessionTaskParams,
) -> Result<LuauExecutionSessionTask, Error> {
    let client = reqwest::Client::new();

    let url = if let Some(version_id) = &params.version_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/versions/{versionId}/luau-execution-sessions/{sessionId}/tasks/{taskId}",
			universeId = &params.universe_id,
			placeId = &params.place_id,
			versionId = version_id,
			sessionId = &params.session_id,
			taskId = &params.task_id,
		)
    } else {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/luau-execution-sessions/{sessionId}/tasks/{taskId}",
			universeId = &params.universe_id,
			placeId = &params.place_id,
			sessionId = &params.session_id,
			taskId = &params.task_id,
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

    let body = res.json::<LuauExecutionSessionTask>().await?;
    Ok(body)
}

pub async fn get_luau_execution_task_logs(
    params: &GetLuauExecutionSessionTaskLogsParams,
) -> Result<LuauExecutionSessionTaskLogPage, Error> {
    let client = reqwest::Client::new();

    let url = if let Some(version_id) = &params.version_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/versions/{versionId}/luau-execution-sessions/{sessionId}/tasks/{taskId}/logs",
			universeId = &params.universe_id,
			placeId = &params.place_id,
			versionId = version_id,
			sessionId = &params.session_id,
			taskId = &params.task_id,
		)
    } else {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/luau-execution-sessions/{sessionId}/tasks/{taskId}/logs",
			universeId = &params.universe_id,
			placeId = &params.place_id,
			sessionId = &params.session_id,
			taskId = &params.task_id,
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

    let body = res.json::<LuauExecutionSessionTaskLogPage>().await?;
    Ok(body)
}
