mod experience;
mod messaging;
mod datastore;
mod error;

pub use experience::PublishVersionType;

use self::{experience::{PublishExperienceParams, PublishExperienceResponse}, messaging::PublishMessageParams, datastore::{ListDataStoresParams, ListDataStoresResponse, ListEntriesResponse, ListEntriesParams}};

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
