pub mod experience;
pub mod messaging;
pub mod datastore;
pub mod error;

pub use experience::PublishVersionType;
use serde::de::DeserializeOwned;

use self::{experience::{PublishExperienceParams, PublishExperienceResponse}, messaging::PublishMessageParams, datastore::{ListDataStoresParams, ListDataStoresResponse, ListEntriesResponse, ListEntriesParams, GetEntryParams, SetEntryResponse, SetEntryParams, IncrementEntryParams, DeleteEntryParams, ListEntryVersionsResponse, ListEntryVersionsParams, GetEntryVersionParams}};

pub struct RbxExperience {
	pub universe_id: u64,
	pub place_id: u64,
	pub api_key: String,
}

impl RbxExperience {
	pub async fn publish(&self, filename: &str, version_type: PublishVersionType) -> anyhow::Result<PublishExperienceResponse> {
		experience::publish_experience(&PublishExperienceParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			place_id: self.place_id,
			version_type,
			filename: filename.to_string(),
		}).await
	}
}

pub struct RbxMessaging {
	pub api_key: String,
	pub universe_id: u64,
	pub topic: String,
}

impl RbxMessaging {
	pub async fn publish(&self, message: String) -> anyhow::Result<()> {
		messaging::publish_message(&PublishMessageParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			topic: self.topic.clone(),
			message,
		}).await
	}
}

pub struct RbxDataStore {
	pub api_key: String,
	pub universe_id: u64,
}

pub struct DataStoreListStores {
	pub prefix: Option<String>,
	pub limit: u64,
	pub cursor: Option<String>,
}

pub struct DataStoreListEntries {
	pub name: String,
	pub scope: Option<String>,
	pub all_scopes: bool,
	pub prefix: Option<String>,
	pub limit: u64,
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
	pub roblox_entry_user_ids: Option<Vec<u64>>,
	pub roblox_entry_attributes: Option<String>,
	pub data: String,
}

pub struct DataStoreIncrementEntry {
	pub name: String,
	pub scope: Option<String>,
	pub key: String,
	pub roblox_entry_user_ids: Option<Vec<u64>>,
	pub roblox_entry_attributes: Option<String>,
	pub increment_by: f64,
}

pub struct DataStoreListEntryVersions {
	pub name: String,
	pub scope: Option<String>,
	pub key: String,
	pub start_time: Option<String>,
	pub end_time: Option<String>,
	pub sort_order: String,
	pub limit: u64,
	pub cursor: Option<String>,
}

pub struct DataStoreGetEntryVersion {
	pub name: String,
	pub scope: Option<String>,
	pub key: String,
	pub version_id: String
}

impl RbxDataStore {
	pub async fn list_stores(&self, params: &DataStoreListStores) -> anyhow::Result<ListDataStoresResponse> {
		datastore::list_datastores(&ListDataStoresParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			prefix: params.prefix.clone(),
			limit: params.limit,
			cursor: params.cursor.clone(),
		}).await
	}

	pub async fn list_entries(&self, params: &DataStoreListEntries) -> anyhow::Result<ListEntriesResponse> {
		datastore::list_entries(&ListEntriesParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: params.name.clone(),
			scope: params.scope.clone(),
			all_scopes: params.all_scopes,
			prefix: params.prefix.clone(),
			limit: params.limit,
			cursor: params.cursor.clone(),
		}).await
	}

	pub async fn get_entry_string(&self, params: &DataStoreGetEntry) -> anyhow::Result<String> {
		datastore::get_entry_string(&GetEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: params.name.clone(),
			scope: params.scope.clone(),
			key: params.key.clone(),
		}).await
	}

	pub async fn get_entry<T: DeserializeOwned>(&self, params: &DataStoreGetEntry) -> anyhow::Result<T> {
		datastore::get_entry::<T>(&GetEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: params.name.clone(),
			scope: params.scope.clone(),
			key: params.key.clone(),
		}).await
	}

	pub async fn set_entry(&self, params: &DataStoreSetEntry) -> anyhow::Result<SetEntryResponse> {
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
		}).await
	}

	pub async fn increment_entry(&self, params: &DataStoreIncrementEntry) -> anyhow::Result<f64> {
		datastore::increment_entry(&IncrementEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: params.name.clone(),
			scope: params.scope.clone(),
			key: params.key.clone(),
			roblox_entry_user_ids: params.roblox_entry_user_ids.clone(),
			roblox_entry_attributes: params.roblox_entry_attributes.clone(),
			increment_by: params.increment_by,
		}).await
	}

	pub async fn delete_entry(&self, name: String, scope: Option<String>, key: String) -> anyhow::Result<()> {
		datastore::delete_entry(&DeleteEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: name,
			scope,
			key,
		}).await
	}

	pub async fn list_entry_versions(&self, params: &DataStoreListEntryVersions) -> anyhow::Result<ListEntryVersionsResponse> {
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
		}).await
	}

	pub async fn get_entry_version(&self, params: &DataStoreGetEntryVersion) -> anyhow::Result<String> {
		datastore::get_entry_version(&GetEntryVersionParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: params.name.clone(),
			scope: params.scope.clone(),
			key: params.key.clone(),
			version_id: params.version_id.clone(),
		}).await
	}
}

#[derive(Debug)]
pub struct RbxCloud {
	pub api_key: String,
	pub universe_id: u64,

}

impl RbxCloud {
	pub fn new(api_key: String, universe_id: u64) -> RbxCloud {
		RbxCloud { api_key, universe_id }
	}

	pub fn experience(&self, place_id: u64) -> RbxExperience {
		RbxExperience { api_key: self.api_key.clone(), universe_id: self.universe_id, place_id }
	}

	pub fn messaging(&self, topic: &str) -> RbxMessaging {
		RbxMessaging { api_key: self.api_key.clone(), universe_id: self.universe_id, topic: topic.to_string() }
	}

	pub fn datastore(&self) -> RbxDataStore {
		RbxDataStore { api_key: self.api_key.clone(), universe_id: self.universe_id }
	}
}
