//! Access into Roblox v2 APIs.
//!
//! Most usage should go through the `Client` struct.

use inventory::{InventoryItems, ListInventoryItemsParams};
use luau_execution::{
    CreateLuauExecutionTaskParams, GetLuauExecutionSessionTaskLogsParams,
    GetLuauExecutionSessionTaskParams, LuauExecutionSessionTask, LuauExecutionSessionTaskLogPage,
    LuauExecutionTaskLogView, NewLuauExecutionSessionTask,
};
use place::{GetPlaceParams, PlaceInfo, UpdatePlaceInfo, UpdatePlaceParams};
use rand::{distr::Alphanumeric, Rng};
use universe::{
    GetUniverseParams, RestartUniverseServersParams, UniverseInfo, UpdateUniverseInfo,
    UpdateUniverseParams,
};
use user::{
    GenerateUserThumbnailOperationResponse, GenerateUserThumbnailParams, GetUserParams,
    GetUserResponse, UserThumbnailFormat, UserThumbnailShape, UserThumbnailSize,
};
use user_restriction::{
    GetUserRestrictionParams, ListUserRestrictionLogsParams, ListUserRestrictionsParams,
    UpdateUserRestrictionParams, UserRestriction, UserRestrictionList, UserRestrictionLogsList,
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
pub mod inventory;
pub mod luau_execution;
pub mod notification;
pub mod place;
pub mod subscription;
pub mod universe;
pub mod user;
pub mod user_restriction;

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

pub struct InventoryClient {
    pub api_key: String,
}

pub struct LuauExecutionClient {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_id: Option<String>,
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

pub struct UserRestrictionClient {
    pub api_key: String,
    pub universe_id: UniverseId,
}

pub struct UserRestrictionParams {
    pub user_id: RobloxUserId,
    pub place_id: Option<PlaceId>,
    pub active: Option<bool>,
    pub duration: Option<u64>,
    pub private_reason: Option<String>,
    pub display_reason: Option<String>,
    pub exclude_alt_accounts: Option<bool>,
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

impl InventoryClient {
    pub async fn list_inventory_items(
        &self,
        user_id: RobloxUserId,
        max_page_size: Option<u32>,
        page_token: Option<String>,
        filter: Option<String>,
    ) -> Result<InventoryItems, Error> {
        inventory::list_inventory_items(&ListInventoryItemsParams {
            api_key: self.api_key.clone(),
            user_id,
            max_page_size,
            page_token,
            filter,
        })
        .await
    }
}

impl LuauExecutionClient {
    pub async fn create_task(
        &self,
        script: String,
        timeout: Option<String>,
    ) -> Result<NewLuauExecutionSessionTask, Error> {
        luau_execution::create_luau_execution_task(&CreateLuauExecutionTaskParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id: self.place_id,
            version_id: self.version_id.clone(),
            script,
            timeout,
        })
        .await
    }

    pub async fn get_task(
        &self,
        session_id: String,
        task_id: String,
    ) -> Result<LuauExecutionSessionTask, Error> {
        luau_execution::get_luau_execution_task(&GetLuauExecutionSessionTaskParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id: self.place_id,
            version_id: self.version_id.clone(),
            session_id,
            task_id,
        })
        .await
    }

    pub async fn get_logs(
        &self,
        session_id: String,
        task_id: String,
        view: LuauExecutionTaskLogView,
        max_page_size: Option<u32>,
        page_token: Option<String>,
    ) -> Result<LuauExecutionSessionTaskLogPage, Error> {
        luau_execution::get_luau_execution_task_logs(&GetLuauExecutionSessionTaskLogsParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id: self.place_id,
            version_id: self.version_id.clone(),
            session_id,
            task_id,
            view,
            max_page_size,
            page_token,
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
            info,
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

impl UserRestrictionClient {
    pub async fn list_user_restrictions(
        &self,
        place_id: Option<PlaceId>,
        max_page_size: Option<u32>,
        filter: Option<String>,
        page_token: Option<String>,
    ) -> Result<UserRestrictionList, Error> {
        user_restriction::list_user_restrictions(&ListUserRestrictionsParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id,
            max_page_size,
            page_token,
            filter,
        })
        .await
    }

    pub async fn get_user_restriction(
        &self,
        user_id: RobloxUserId,
        place_id: Option<PlaceId>,
    ) -> Result<UserRestriction, Error> {
        user_restriction::get_user_restriction(&GetUserRestrictionParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id,
            user_id,
        })
        .await
    }

    pub async fn update_user_restriction(
        &mut self,
        params: &UserRestrictionParams,
    ) -> Result<UserRestriction, Error> {
        let idempotency_key: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        user_restriction::update_user_restriction(&UpdateUserRestrictionParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id: params.place_id,
            user_id: params.user_id,
            idempotency_key: Some(idempotency_key),
            active: params.active,
            duration: params.duration.and_then(|d| Some(format!("{}s", d))),
            private_reason: params.private_reason.clone(),
            display_reason: params.display_reason.clone(),
            exclude_alt_accounts: params.exclude_alt_accounts,
        })
        .await
    }

    pub async fn list_user_restriction_logs(
        &self,
        place_id: Option<PlaceId>,
        max_page_size: Option<u32>,
        page_token: Option<String>,
        filter: Option<String>,
    ) -> Result<UserRestrictionLogsList, Error> {
        user_restriction::list_user_restriction_logs(&ListUserRestrictionLogsParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id,
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

    pub fn inventory(&self) -> InventoryClient {
        InventoryClient {
            api_key: self.api_key.clone(),
        }
    }

    pub fn luau(
        &self,
        universe_id: UniverseId,
        place_id: PlaceId,
        version_id: Option<String>,
    ) -> LuauExecutionClient {
        LuauExecutionClient {
            api_key: self.api_key.clone(),
            universe_id,
            place_id,
            version_id,
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

    pub fn user_restriction(&self, universe_id: UniverseId) -> UserRestrictionClient {
        UserRestrictionClient {
            api_key: self.api_key.clone(),
            universe_id,
        }
    }
}
