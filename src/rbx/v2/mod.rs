//! Access into Roblox v2 APIs.
//!
//! Most usage should go through the `Client` struct.

use self::group::{
    GetGroupParams, GetGroupResponse, GetGroupShoutParams, GetGroupShoutResponse, GroupId,
    ListGroupMembershipsParams, ListGroupMembershipsResponse, ListGroupRolesParams,
    ListGroupRolesResponse,
};
pub mod group;
pub(crate) mod http_err;

use crate::rbx::error::Error;

/// Access into the Roblox Open Cloud APIs.
///
/// ```rust,no_run
/// use rbxcloud::rbx::v2::Client;
///
/// let client = RbxCloud::new("API_KEY");
/// ```
#[derive(Debug)]
pub struct Client {
    /// Roblox API key.
    pub api_key: String,
}

pub struct GroupClient {
    pub api_key: String,
    pub group_id: GroupId,
}

impl GroupClient {
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

    pub async fn list_roles(
        &self,
        max_page_size: Option<u32>,
        page_token: Option<String>,
    ) -> Result<ListGroupRolesResponse, Error> {
        group::list_group_roles(&ListGroupRolesParams {
            api_key: self.api_key.clone(),
            group_id: self.group_id,
            max_page_size,
            page_token,
        })
        .await
    }

    pub async fn list_memberships(
        &self,
        max_page_size: Option<u32>,
        filter: Option<String>,
        page_token: Option<String>,
    ) -> Result<ListGroupMembershipsResponse, Error> {
        group::list_group_memberships(&ListGroupMembershipsParams {
            api_key: self.api_key.clone(),
            group_id: self.group_id,
            max_page_size,
            page_token,
            filter,
        })
        .await
    }
}

impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_string(),
        }
    }

    pub fn group(&self, group_id: GroupId) -> GroupClient {
        GroupClient {
            api_key: self.api_key.clone(),
            group_id,
        }
    }
}
