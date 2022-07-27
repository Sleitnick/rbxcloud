pub mod experience;
pub mod messaging;
pub mod datastore;
pub mod error;

pub use experience::PublishVersionType;
use serde::de::DeserializeOwned;

use self::{experience::{PublishExperienceParams, PublishExperienceResponse}, messaging::PublishMessageParams, datastore::{ListDataStoresParams, ListDataStoresResponse, ListEntriesResponse, ListEntriesParams, GetEntryParams, SetEntryResponse, SetEntryParams, IncrementEntryParams, DeleteEntryParams, ListEntryVersionsResponse, ListEntryVersionsParams}};

pub struct RbxExperience {
	pub universe_id: u64,
	pub place_id: u64,
	pub api_key: String,
}

impl RbxExperience {
	pub async fn publish(&self, filename: &String, version_type: PublishVersionType) -> anyhow::Result<PublishExperienceResponse> {
		experience::publish_experience(&PublishExperienceParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			place_id: self.place_id,
			version_type: version_type,
			filename: filename.clone(),
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
			message: message,
		}).await
	}
}

pub struct RbxDataStore {
	pub api_key: String,
	pub universe_id: u64,
}

impl RbxDataStore {
	pub async fn list_stores(&self, prefix: Option<String>, limit: u64, cursor: Option<String>) -> anyhow::Result<ListDataStoresResponse> {
		datastore::list_datastores(&ListDataStoresParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			prefix: prefix,
			limit: limit,
			cursor: cursor,
		}).await
	}

	pub async fn list_entries(&self, name: String, scope: Option<String>, all_scopes: bool, prefix: Option<String>, limit: u64, cursor: Option<String>) -> anyhow::Result<ListEntriesResponse> {
		datastore::list_entries(&ListEntriesParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: name,
			scope: scope,
			all_scopes: all_scopes,
			prefix: prefix,
			limit: limit,
			cursor: cursor,
		}).await
	}

	pub async fn get_entry_string(&self, name: String, scope: Option<String>, key: String) -> anyhow::Result<String> {
		datastore::get_entry_string(&GetEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: name,
			scope: scope,
			key: key,
		}).await
	}

	pub async fn get_entry<T: DeserializeOwned>(&self, name: String, scope: Option<String>, key: String) -> anyhow::Result<T> {
		datastore::get_entry::<T>(&GetEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: name,
			scope: scope,
			key: key,
		}).await
	}

	pub async fn set_entry(&self, name: String, scope: Option<String>, key: String, match_version: Option<String>, exclusive_create: Option<bool>, roblox_entry_user_ids: Option<Vec<u64>>, roblox_entry_attributes: Option<String>, data: String) -> anyhow::Result<SetEntryResponse> {
		datastore::set_entry(&SetEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: name,
			scope: scope,
			key: key,
			match_version: match_version,
			exclusive_create: exclusive_create,
			roblox_entry_user_ids: roblox_entry_user_ids,
			roblox_entry_attributes: roblox_entry_attributes,
			data: data,
		}).await
	}

	pub async fn increment_entry(&self, name: String, scope: Option<String>, key: String, roblox_entry_user_ids: Option<Vec<u64>>, roblox_entry_attributes: Option<String>, increment: f64) -> anyhow::Result<f64> {
		datastore::increment_entry(&IncrementEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: name,
			scope: scope,
			key: key,
			roblox_entry_user_ids: roblox_entry_user_ids,
			roblox_entry_attributes: roblox_entry_attributes,
			increment_by: increment,
		}).await
	}

	pub async fn delete_entry(&self, name: String, scope: Option<String>, key: String) -> anyhow::Result<()> {
		datastore::delete_entry(&DeleteEntryParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: name,
			scope: scope,
			key: key,
		}).await
	}

	pub async fn list_entry_versions(&self, name: String, scope: Option<String>, key: String, start_time: Option<String>, end_time: Option<String>, sort_order: String, limit: u64, cursor: Option<String>) -> anyhow::Result<ListEntryVersionsResponse> {
		datastore::list_entry_versions(&ListEntryVersionsParams {
			api_key: self.api_key.clone(),
			universe_id: self.universe_id,
			datastore_name: name,
			scope: scope,
			key: key,
			start_time: start_time,
			end_time: end_time,
			sort_order: sort_order,
			limit: limit,
			cursor: cursor,
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
		RbxExperience { api_key: self.api_key.clone(), universe_id: self.universe_id.clone(), place_id }
	}

	pub fn messaging(&self, topic: &String) -> RbxMessaging {
		RbxMessaging { api_key: self.api_key.clone(), universe_id: self.universe_id.clone(), topic: topic.clone() }
	}

	pub fn datastore(&self) -> RbxDataStore {
		RbxDataStore { api_key: self.api_key.clone(), universe_id: self.universe_id.clone() }
	}
}
