//! Access into Roblox v1 APIs.
//!
//! Most usage should go through the `RbxCloud` struct.
pub mod assets;
pub mod datastore;
pub(crate) mod ds_error;
pub mod experience;
pub mod messaging;
pub mod ordered_datastore;

use crate::rbx::error;
pub use experience::PublishVersionType;
use serde::de::DeserializeOwned;

use self::{
    assets::{
        AssetCreation, AssetGetOperation, AssetOperation, AssetType, CreateAssetParams,
        CreateAssetParamsWithContents, GetAssetParams, UpdateAssetParams,
    },
    datastore::{
        DeleteEntryParams, GetEntryParams, GetEntryVersionParams, IncrementEntryParams,
        ListDataStoresParams, ListDataStoresResponse, ListEntriesParams, ListEntriesResponse,
        ListEntryVersionsParams, ListEntryVersionsResponse, SetEntryParams, SetEntryResponse,
    },
    error::Error,
    experience::{PublishExperienceParams, PublishExperienceResponse},
    messaging::PublishMessageParams,
    ordered_datastore::{
        OrderedCreateEntryParams, OrderedEntry, OrderedEntryParams, OrderedIncrementEntryParams,
        OrderedListEntriesParams, OrderedListEntriesResponse, OrderedUpdateEntryParams,
    },
};

use super::types::{PageSize, PlaceId, ReturnLimit, RobloxUserId, UniverseId};

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

impl std::fmt::Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<u64> for PageSize {
    fn from(item: u64) -> Self {
        PageSize(item)
    }
}

pub struct RbxExperience {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub api_key: String,
}

impl RbxExperience {
    /// Publish a place.
    ///
    /// The filename should point to a `*.rbxl` or `*.rbxlx` file.
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
    /// Publish a message.
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
    /// List DataStores within the experience.
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

    /// List key entries in a specific DataStore.
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

    /// Get the entry string representation of a specific key.
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

    /// Get the entry of a specific key, deserialized as `T`.
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

    /// Set (or create) the entry value of a specific key.
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

    /// Increment (or create) the value of a specific key.
    ///
    /// If the value does not yet exist, it will be treated as `0`, and thus
    /// the resulting value will simply be the increment amount.
    ///
    /// If the value _does_ exist, but it is _not_ a number, then the increment
    /// process will fail, and a DataStore error will be returned in the result.
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

    /// Delete an entry.
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

    /// List all versions of an entry.
    ///
    /// To get the specific value of a given entry, use `get_entry_version()`.
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

    /// Get the entry value of a specific version.
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

pub struct RbxOrderedDataStore {
    pub api_key: String,
    pub universe_id: UniverseId,
}

pub struct OrderedDataStoreListEntries {
    pub name: String,
    pub scope: Option<String>,
    pub max_page_size: Option<PageSize>,
    pub page_token: Option<String>,
    pub order_by: Option<String>,
    pub filter: Option<String>,
}

pub struct OrderedDataStoreCreateEntry {
    pub name: String,
    pub scope: Option<String>,
    pub id: String,
    pub value: i64,
}

pub struct OrderedDataStoreUpdateEntry {
    pub name: String,
    pub scope: Option<String>,
    pub id: String,
    pub value: i64,
    pub allow_missing: Option<bool>,
}

pub struct OrderedDataStoreIncrementEntry {
    pub name: String,
    pub scope: Option<String>,
    pub id: String,
    pub increment: i64,
}

pub struct OrderedDataStoreEntry {
    pub name: String,
    pub scope: Option<String>,
    pub id: String,
}

impl RbxOrderedDataStore {
    /// List key entries
    pub async fn list_entries(
        &self,
        params: &OrderedDataStoreListEntries,
    ) -> Result<OrderedListEntriesResponse, Error> {
        ordered_datastore::list_entries(&OrderedListEntriesParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            ordered_datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            max_page_size: params.max_page_size,
            page_token: params.page_token.clone(),
            order_by: params.order_by.clone(),
            filter: params.filter.clone(),
        })
        .await
    }

    /// Create an entry
    pub async fn create_entry(
        &self,
        params: &OrderedDataStoreCreateEntry,
    ) -> Result<OrderedEntry, Error> {
        ordered_datastore::create_entry(&OrderedCreateEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            ordered_datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            id: params.id.to_string(),
            value: params.value,
        })
        .await
    }

    /// Get an entry
    pub async fn get_entry(&self, params: &OrderedDataStoreEntry) -> Result<OrderedEntry, Error> {
        ordered_datastore::get_entry(&OrderedEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            ordered_datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            id: params.id.to_string(),
        })
        .await
    }

    /// Delete an entry
    pub async fn delete_entry(&self, params: &OrderedDataStoreEntry) -> Result<(), Error> {
        ordered_datastore::delete_entry(&OrderedEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            ordered_datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            id: params.id.to_string(),
        })
        .await
    }

    /// Update an entry
    pub async fn update_entry(
        &self,
        params: &OrderedDataStoreUpdateEntry,
    ) -> Result<OrderedEntry, Error> {
        ordered_datastore::update_entry(&OrderedUpdateEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            ordered_datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            id: params.id.to_string(),
            value: params.value,
            allow_missing: params.allow_missing,
        })
        .await
    }

    /// Increment an entry
    pub async fn increment_entry(
        &self,
        params: &OrderedDataStoreIncrementEntry,
    ) -> Result<OrderedEntry, Error> {
        ordered_datastore::increment_entry(&OrderedIncrementEntryParams {
            api_key: self.api_key.clone(),
            universe_id: self.universe_id,
            ordered_datastore_name: params.name.clone(),
            scope: params.scope.clone(),
            id: params.id.to_string(),
            increment: params.increment,
        })
        .await
    }
}

pub struct RbxAssets {
    /// Roblox API key.
    pub api_key: String,
}

pub struct CreateAsset {
    pub asset: AssetCreation,
    pub filepath: String,
}

pub struct CreateAssetWithContents<'a> {
    pub asset: AssetCreation,
    pub contents: &'a [u8],
}

pub struct UpdateAsset {
    pub asset_id: u64,
    pub asset_type: AssetType,
    pub filepath: String,
}

pub struct GetAsset {
    pub operation_id: String,
}

impl RbxAssets {
    /// Create an asset
    pub async fn create(&self, params: &CreateAsset) -> Result<AssetOperation, Error> {
        assets::create_asset(&CreateAssetParams {
            api_key: self.api_key.clone(),
            asset: params.asset.clone(),
            filepath: params.filepath.clone(),
        })
        .await
    }

    pub async fn create_with_contents<'a>(
        &self,
        params: &CreateAssetWithContents<'a>,
    ) -> Result<AssetOperation, Error> {
        assets::create_asset_with_contents(&CreateAssetParamsWithContents {
            api_key: self.api_key.clone(),
            asset: params.asset.clone(),
            contents: params.contents,
        })
        .await
    }

    /// Update an asset
    pub async fn update(&self, params: &UpdateAsset) -> Result<AssetOperation, Error> {
        assets::update_asset(&UpdateAssetParams {
            api_key: self.api_key.clone(),
            asset_id: params.asset_id,
            asset_type: params.asset_type,
            filepath: params.filepath.clone(),
        })
        .await
    }

    /// Get asset information
    pub async fn get(&self, params: &GetAsset) -> Result<AssetGetOperation, Error> {
        assets::get_asset(&GetAssetParams {
            api_key: self.api_key.clone(),
            operation_id: params.operation_id.clone(),
        })
        .await
    }
}

/// Access into the Roblox Open Cloud APIs.
///
/// ```rust,no_run
/// use rbxcloud::rbx::v1::RbxCloud;
///
/// let cloud = RbxCloud::new("API_KEY");
/// ```
#[derive(Debug)]
pub struct RbxCloud {
    /// Roblox API key.
    pub api_key: String,
}

impl RbxCloud {
    pub fn new(api_key: &str) -> RbxCloud {
        RbxCloud {
            api_key: api_key.to_string(),
        }
    }

    pub fn assets(&self) -> RbxAssets {
        RbxAssets {
            api_key: self.api_key.clone(),
        }
    }

    pub fn experience(&self, universe_id: UniverseId, place_id: PlaceId) -> RbxExperience {
        RbxExperience {
            api_key: self.api_key.clone(),
            universe_id,
            place_id,
        }
    }

    pub fn messaging(&self, universe_id: UniverseId, topic: &str) -> RbxMessaging {
        RbxMessaging {
            api_key: self.api_key.clone(),
            universe_id,
            topic: topic.to_string(),
        }
    }

    pub fn datastore(&self, universe_id: UniverseId) -> RbxDataStore {
        RbxDataStore {
            api_key: self.api_key.clone(),
            universe_id,
        }
    }

    pub fn ordered_datastore(&self, universe_id: UniverseId) -> RbxOrderedDataStore {
        RbxOrderedDataStore {
            api_key: self.api_key.clone(),
            universe_id,
        }
    }
}
