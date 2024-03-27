//! Access into Roblox v2 APIs.
//!
//! Most usage should go through the `RbxCloud` struct.

use self::group::{
    GetGroupParams, GetGroupResponse, GetGroupShoutParams, GetGroupShoutResponse, GroupId,
};
pub mod group;
pub(crate) mod http_err;

use crate::rbx::error::Error;

/// Access into the Roblox Open Cloud APIs.
///
/// ```rust,no_run
/// use rbxcloud::rbx::v2::RbxCloud;
///
/// let cloud = RbxCloud::new("API_KEY");
/// ```
#[derive(Debug)]
pub struct RbxCloud {
    /// Roblox API key.
    pub api_key: String,
}

pub struct RbxGroup {
    pub api_key: String,
    pub group_id: GroupId,
}

impl RbxGroup {
    pub async fn get_info(&self) -> Result<GetGroupResponse, Error> {
        group::get_group(&GetGroupParams {
            api_key: self.api_key.clone(),
            group_id: self.group_id,
        })
        .await
    }

    pub async fn get_shout(&self) -> Result<GetGroupShoutResponse, Error> {
        group::get_group_shout(&GetGroupShoutParams {
            api_key: self.api_key.clone(),
            group_id: self.group_id,
        })
        .await
    }
}

impl RbxCloud {
    pub fn new(api_key: &str) -> RbxCloud {
        RbxCloud {
            api_key: api_key.to_string(),
        }
    }

    pub fn group(&self, group_id: GroupId) -> RbxGroup {
        RbxGroup {
            api_key: self.api_key.clone(),
            group_id,
        }
    }
}
