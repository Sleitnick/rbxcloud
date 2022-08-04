//! Low-level Experience API operations.
use std::fmt;

use serde::Deserialize;

use crate::rbx::error::Error;

use super::{PlaceId, UniverseId};

/// The version type of a place publish operation.
#[derive(Debug, Clone)]
pub enum PublishVersionType {
    /// Place is saved as the most-recent version. Players who play
    /// the game will _not_ see this version, but it _will_ be the
    /// version that is seen when loaded in with Studio.
    Saved,

    /// Place is saved as the most-recent live version. Players who
    /// play the game will play this version.
    Published,
}

impl fmt::Display for PublishVersionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Parameters for publishing a place.
pub struct PublishExperienceParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_type: PublishVersionType,
    pub filename: String,
}

/// Response from Roblox when place is published or saved.
///
/// The version number represents the latest version uploaded
/// to Roblox.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublishExperienceResponse {
    pub version_number: u64,
}

/// Publish a place under a specific experience.
pub async fn publish_experience(
    params: &PublishExperienceParams,
) -> Result<PublishExperienceResponse, Error> {
    let client = reqwest::Client::new();
    let bytes_data_buf = std::fs::read(&params.filename)?;
    let url = format!(
        "https://apis.roblox.com/universes/v1/{universeId}/places/{placeId}/versions?versionType={versionType}",
        universeId=params.universe_id,
        placeId=params.place_id,
        versionType=params.version_type,
    );
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .header("Content-Type", "application/octet-stream")
        .body(bytes_data_buf)
        .send()
        .await?;
    let status = res.status();
    if !status.is_success() {
        let code = status.as_u16();
        if code == 400 {
            return Err(Error::HttpStatusError {
                code,
                msg: "invalid request or file content".to_string(),
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
        } else if code == 404 {
            return Err(Error::HttpStatusError {
                code,
                msg: "place or universe does not exist".to_string(),
            });
        } else if code == 409 {
            return Err(Error::HttpStatusError {
                code,
                msg: "place not part of the universe".to_string(),
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
    let body = res.json::<PublishExperienceResponse>().await?;
    Ok(body)
}
