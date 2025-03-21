//! Access into Roblox v2 APIs.
//!
//! Most usage should go through the `Client` struct.

use place::{GetPlaceParams, PlaceInfo, UpdatePlaceInfo, UpdatePlaceParams};
use universe::{
    GetUniverseParams, RestartUniverseServersParams, UniverseInfo, UpdateUniverseInfo,
    UpdateUniverseParams,
};
use user::{
    GenerateUserThumbnailOperationResponse, GenerateUserThumbnailParams, GetUserParams,
    GetUserResponse, UserThumbnailFormat, UserThumbnailShape, UserThumbnailSize,
};

use self::{
    group::{
        GetGroupParams, GetGroupResponse, GetGroupShoutParams, GetGroupShoutResponse,
        ListGroupMembershipsParams, ListGroupMembershipsResponse, ListGroupRolesParams,
        ListGroupRolesResponse,
    },
    notification::{Notification, NotificationParams, NotificationResponse},
    subscription::{GetSubscriptionParams, GetSubscriptionResponse, SubscriptionView},
};
pub mod group;
pub(crate) mod http_err;
pub mod luau_execution;
pub mod notification;
pub mod place;
pub mod subscription;
pub mod universe;
pub mod user;

use crate::rbx::error::Error;

use super::types::{GroupId, PlaceId, RobloxUserId, UniverseId};

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

pub struct NotificationClient {
    pub api_key: String,
    pub universe_id: UniverseId,
}

pub struct PlaceClient {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
}

pub struct UniverseClient {
    pub api_key: String,
    pub universe_id: UniverseId,
}

pub struct UserClient {
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

impl NotificationClient {
    pub async fn send(
        &self,
        user_id: RobloxUserId,
        notification: Notification,
    ) -> Result<NotificationResponse, Error> {
        notification::send_notification(&NotificationParams {
            api_key: self.api_key.clone(),
            user_id,
            notification,
        })
        .await
    }
}

impl PlaceClient {
    pub async fn get(&self) -> Result<PlaceInfo, Error> {
        place::get_place(&GetPlaceParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id: self.place_id,
        })
        .await
    }

    pub async fn update(
        &self,
        update_mask: String,
        info: UpdatePlaceInfo,
    ) -> Result<PlaceInfo, Error> {
        place::update_place(&UpdatePlaceParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id: self.place_id,
            update_mask,
            info,
        })
        .await
    }
}

impl UniverseClient {
    pub async fn get(&self) -> Result<UniverseInfo, Error> {
        universe::get_universe(&GetUniverseParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
        })
        .await
    }

    pub async fn update(
        &self,
        update_mask: String,
        info: UpdateUniverseInfo,
    ) -> Result<UniverseInfo, Error> {
        universe::update_universe(&UpdateUniverseParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            update_mask,
            info: info,
        })
        .await
    }

    pub async fn restart_servers(&self) -> Result<(), Error> {
        universe::restart_universe_servers(&RestartUniverseServersParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
        })
        .await
    }
}

impl UserClient {
    pub async fn get_user(&self, user_id: RobloxUserId) -> Result<GetUserResponse, Error> {
        user::get_user(&GetUserParams {
            api_key: self.api_key.clone(),
            user_id,
        })
        .await
    }

    pub async fn generate_thumbnail(
        &self,
        user_id: RobloxUserId,
        size: Option<UserThumbnailSize>,
        format: Option<UserThumbnailFormat>,
        shape: Option<UserThumbnailShape>,
    ) -> Result<GenerateUserThumbnailOperationResponse, Error> {
        user::generate_thumbnail(&GenerateUserThumbnailParams {
            api_key: self.api_key.clone(),
            user_id,
            size,
            shape,
            format,
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

    pub fn notification(&self, universe_id: UniverseId) -> NotificationClient {
        NotificationClient {
            api_key: self.api_key.clone(),
            universe_id,
        }
    }

    pub fn place(&self, universe_id: UniverseId, place_id: PlaceId) -> PlaceClient {
        PlaceClient {
            api_key: self.api_key.clone(),
            universe_id,
            place_id,
        }
    }

    pub fn universe(&self, universe_id: UniverseId) -> UniverseClient {
        UniverseClient {
            api_key: self.api_key.clone(),
            universe_id,
        }
    }

    pub fn user(&self) -> UserClient {
        UserClient {
            api_key: self.api_key.clone(),
        }
    }
}
