pub mod datastore;
pub mod error;
pub mod experience;
pub mod messaging;

pub use experience::PublishVersionType;
use serde::de::DeserializeOwned;

use self::{
    datastore::{
        DeleteEntryParams, GetEntryParams, GetEntryVersionParams, IncrementEntryParams,
        ListDataStoresParams, ListDataStoresResponse, ListEntriesParams, ListEntriesResponse,
        ListEntryVersionsParams, ListEntryVersionsResponse, SetEntryParams, SetEntryResponse,
    },
    error::Error,
    experience::{PublishExperienceParams, PublishExperienceResponse},
    messaging::PublishMessageParams,
};

#[derive(Debug, Clone, Copy)]
pub struct UniverseId(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct PlaceId(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct ReturnLimit(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct RobloxUserId(pub u64);

// impl FromIterator<u64> for RobloxUserId {
//     fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
//         let mut c = vec![];
//         for i in iter {
//             c.push(i);
//         }
//         c
//     }
// }

impl std::fmt::Display for UniverseId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for PlaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for ReturnLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for RobloxUserId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct RbxExperience {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub api_key: String,
}

impl RbxExperience {
    pub async fn publish(
        &self,
        filename: &str,
        version_type: PublishVersionType,
    ) -> Result<PublishExperienceResponse, Error> {
        experience::publish_experience(&PublishExperienceParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id: self.place_id,
            version_type,
            filename: filename.to_string(),
        })
        .await
    }
}

pub struct RbxMessaging {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub topic: String,
}

impl RbxMessaging {
    pub async fn publish(&self, message: &str) -> Result<(), Error> {
        messaging::publish_message(&PublishMessageParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            topic: self.topic.clone(),
            message: message.to_string(),
        })
        .await
    }
}

pub struct RbxDataStore {
    pub api_key: String,
    pub universe_id: UniverseId,
}

pub struct DataStoreListStores {
    pub prefix: Option<String>,
    pub limit: ReturnLimit,
    pub cursor: Option<String>,
}

pub struct DataStoreListEntries {
    pub name: String,
    pub scope: Option<String>,
    pub all_scopes: bool,
    pub prefix: Option<String>,
    pub limit: ReturnLimit,
    pub cursor: Option<String>,
}

pub struct DataStoreGetEntry {
    pub name: String,
    pub scope: Option<String>,
    pub key: String,
}

pub struct DataStoreSetEntry {
    pub name: String,
    pub scope: Option<String>,
    pub key: String,
    pub match_version: Option<String>,
    pub exclusive_create: Option<bool>,
    pub roblox_entry_user_ids: Option<Vec<RobloxUserId>>,
    pub roblox_entry_attributes: Option<String>,
    pub data: String,
}

pub struct DataStoreIncrementEntry {
    pub name: String,
    pub scope: Option<String>,
    pub key: String,
    pub roblox_entry_user_ids: Option<Vec<RobloxUserId>>,
    pub roblox_entry_attributes: Option<String>,
    pub increment_by: f64,
}

pub struct DataStoreDeleteEntry {
    pub name: String,
    pub scope: Option<String>,
    pub key: String,
}

pub struct DataStoreListEntryVersions {
    pub name: String,
    pub scope: Option<String>,
    pub key: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub sort_order: String,
    pub limit: ReturnLimit,
    pub cursor: Option<String>,
}

pub struct DataStoreGetEntryVersion {
    pub name: String,
    pub scope: Option<String>,
    pub key: String,
    pub version_id: String,
}

impl RbxDataStore {
    pub async fn list_stores(
        &self,
        params: &DataStoreListStores,
    ) -> Result<ListDataStoresResponse, Error> {
        datastore::list_datastores(&ListDataStoresParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            prefix: params.prefix.clone(),
            limit: params.limit,
            cursor: params.cursor.clone(),
        })
        .await
    }

    pub async fn list_entries(
        &self,
        params: &DataStoreListEntries,
    ) -> Result<ListEntriesResponse, Error> {
        datastore::list_entries(&ListEntriesParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            all_scopes: params.all_scopes,
            prefix: params.prefix.clone(),
            limit: params.limit,
            cursor: params.cursor.clone(),
        })
        .await
    }

    pub async fn get_entry_string(&self, params: &DataStoreGetEntry) -> Result<String, Error> {
        datastore::get_entry_string(&GetEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            key: params.key.clone(),
        })
        .await
    }

    pub async fn get_entry<T: DeserializeOwned>(
        &self,
        params: &DataStoreGetEntry,
    ) -> Result<T, Error> {
        datastore::get_entry::<T>(&GetEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            key: params.key.clone(),
        })
        .await
    }

    pub async fn set_entry(&self, params: &DataStoreSetEntry) -> Result<SetEntryResponse, Error> {
        datastore::set_entry(&SetEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            key: params.key.clone(),
            match_version: params.match_version.clone(),
            exclusive_create: params.exclusive_create,
            roblox_entry_user_ids: params.roblox_entry_user_ids.clone(),
            roblox_entry_attributes: params.roblox_entry_attributes.clone(),
            data: params.data.clone(),
        })
        .await
    }

    pub async fn increment_entry(&self, params: &DataStoreIncrementEntry) -> Result<f64, Error> {
        datastore::increment_entry(&IncrementEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            key: params.key.clone(),
            roblox_entry_user_ids: params.roblox_entry_user_ids.clone(),
            roblox_entry_attributes: params.roblox_entry_attributes.clone(),
            increment_by: params.increment_by,
        })
        .await
    }

    pub async fn delete_entry(&self, params: &DataStoreDeleteEntry) -> Result<(), Error> {
        datastore::delete_entry(&DeleteEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            key: params.key.clone(),
        })
        .await
    }

    pub async fn list_entry_versions(
        &self,
        params: &DataStoreListEntryVersions,
    ) -> Result<ListEntryVersionsResponse, Error> {
        datastore::list_entry_versions(&ListEntryVersionsParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            key: params.key.clone(),
            start_time: params.start_time.clone(),
            end_time: params.end_time.clone(),
            sort_order: params.sort_order.clone(),
            limit: params.limit,
            cursor: params.cursor.clone(),
        })
        .await
    }

    pub async fn get_entry_version(
        &self,
        params: &DataStoreGetEntryVersion,
    ) -> Result<String, Error> {
        datastore::get_entry_version(&GetEntryVersionParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            key: params.key.clone(),
            version_id: params.version_id.clone(),
        })
        .await
    }
}

#[derive(Debug)]
pub struct RbxCloud {
    pub api_key: String,
    pub universe_id: UniverseId,
}

impl RbxCloud {
    pub fn new(api_key: &str, universe_id: UniverseId) -> RbxCloud {
        RbxCloud {
            api_key: api_key.to_string(),
            universe_id,
        }
    }

    pub fn experience(&self, place_id: PlaceId) -> RbxExperience {
        RbxExperience {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            place_id,
        }
    }

    pub fn messaging(&self, topic: &str) -> RbxMessaging {
        RbxMessaging {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            topic: topic.to_string(),
        }
    }

    pub fn datastore(&self) -> RbxDataStore {
        RbxDataStore {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
        }
    }
}
