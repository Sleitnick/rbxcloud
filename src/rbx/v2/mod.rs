//! Access into Roblox v2 APIs.
//!
//! Most usage should go through the `Client` struct.

use self::{
    group::{
        GetGroupParams, GetGroupResponse, GetGroupShoutParams, GetGroupShoutResponse,
        ListGroupMembershipsParams, ListGroupMembershipsResponse, ListGroupRolesParams,
        ListGroupRolesResponse,
    },
    subscription::{GetSubscriptionParams, GetSubscriptionResponse, SubscriptionView},
};
pub mod group;
pub(crate) mod http_err;
pub mod subscription;

use crate::rbx::error::Error;

use super::types::{GroupId, UniverseId};

/// Access into the Roblox Open Cloud APIs.
///
/// ```rust,no_run
/// use rbxcloud::rbx::v2::Client;
///
/// let client = Client::new("API_KEY");
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

pub struct SubscriptionClient {
    pub api_key: String,
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

impl SubscriptionClient {
    pub async fn get(
        &self,
        universe_id: UniverseId,
        subscription_product: String,
        subscription: String,
        view: Option<SubscriptionView>,
    ) -> Result<GetSubscriptionResponse, Error> {
        subscription::get_subscription(&GetSubscriptionParams {
            api_key: self.api_key.clone(),
            universe_id,
            subscription,
            subscription_product,
            view,
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

    pub fn subscription(&self) -> SubscriptionClient {
        SubscriptionClient {
            api_key: self.api_key.clone(),
        }
    }
}
