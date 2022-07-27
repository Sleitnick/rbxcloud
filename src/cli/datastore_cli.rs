use clap::{Subcommand, Args, ValueEnum};

use crate::rbx::RbxCloud;

#[derive(Debug, Subcommand)]
pub enum DataStoreCommands {
	ListStores {
		/// Return only DataStores with this prefix
		#[clap(short, long, value_parser)]
		prefix: Option<String>,

		/// Maximum number of items to return
		#[clap(short, long, value_parser)]
		limit: u64,

		/// Cursor for the next set of data
		#[clap(short, long, value_parser)]
		cursor: Option<String>,
    
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
	},

	List {
		/// DataStore name
		#[clap(short, long, value_parser)]
		datastore_name: String,

		/// DataStore scope
		#[clap(short, long, value_parser)]
		scope: Option<String>,

		/// If true, return keys from all scopes
		#[clap(short = 'o', long, value_parser)]
		all_scopes: bool,

		/// Return only DataStores with this prefix
		#[clap(short, long, value_parser)]
		prefix: Option<String>,

		/// Maximum number of items to return
		#[clap(short, long, value_parser)]
		limit: u64,

		/// Cursor for the next set of data
		#[clap(short, long, value_parser)]
		cursor: Option<String>,
    
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
	},

	Get {
		/// DataStore name
		#[clap(short, long, value_parser)]
		datastore_name: String,

		/// DataStore scope
		#[clap(short, long, value_parser)]
		scope: Option<String>,

		/// The key of the entry
		#[clap(short, long, value_parser)]
		key: String,
    
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
	},

	Set {
		/// DataStore name
		#[clap(short, long, value_parser)]
		datastore_name: String,

		/// DataStore scope
		#[clap(short, long, value_parser)]
		scope: Option<String>,

		/// The key of the entry
		#[clap(short, long, value_parser)]
		key: String,

		/// Only update if the current version matches this
		#[clap(short = 'i', long, value_parser)]
		match_version: Option<String>,

		/// Only create the entry if it does not exist
		#[clap(short, long, value_parser)]
		exclusive_create: Option<bool>,

		/// JSON-stringified data (up to 4MB)
		#[clap(short = 'D', long, value_parser)]
		data: String,

		/// Associated UserID (can be multiple)
		#[clap(short = 'U', long, value_parser)]
		user_ids: Option<Vec<u64>>,

		/// JSON-stringified attributes data
		#[clap(short = 't', long, value_parser)]
		attributes: Option<String>,
    
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
	},

	Increment {
		/// DataStore name
		#[clap(short, long, value_parser)]
		datastore_name: String,

		/// DataStore scope
		#[clap(short, long, value_parser)]
		scope: Option<String>,

		/// The key of the entry
		#[clap(short, long, value_parser)]
		key: String,

		/// The amount by which the entry should be incremented
		#[clap(short, long, value_parser)]
		increment_by: f64,

		/// Comma-separated list of Roblox user IDs
		#[clap(short = 'U', long, value_parser)]
		user_ids: Option<Vec<u64>>,

		/// JSON-stringified attributes data
		#[clap(short = 't', long, value_parser)]
		attributes: Option<String>,
    
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
	},

	Delete {
		/// DataStore name
		#[clap(short, long, value_parser)]
		datastore_name: String,

		/// DataStore scope
		#[clap(short, long, value_parser)]
		scope: Option<String>,

		/// The key of the entry
		#[clap(short, long, value_parser)]
		key: String,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
	},

	ListVersions {
		/// DataStore name
		#[clap(short, long, value_parser)]
		datastore_name: String,

		/// DataStore scope
		#[clap(short, long, value_parser)]
		scope: Option<String>,

		/// The key of the entry
		#[clap(short, long, value_parser)]
		key: String,

		/// Start time constraint (ISO UTC Datetime)
		#[clap(short = 't', long, value_parser)]
		start_time: Option<String>,

		/// End time constraint (ISO UTC Datetime)
		#[clap(short = 'e', long, value_parser)]
		end_time: Option<String>,

		/// Sort order
		#[clap(short = 'o', long, value_enum)]
		sort_order: ListEntrySortOrder,

		/// Maximum number of items to return
		#[clap(short, long, value_parser)]
		limit: u64,

		/// Cursor for the next set of data
		#[clap(short, long, value_parser)]
		cursor: Option<String>,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
	},

	GetVersion {
		/// DataStore name
		#[clap(short, long, value_parser)]
		datastore_name: String,

		/// DataStore scope
		#[clap(short, long, value_parser)]
		scope: Option<String>,

		/// The key of the entry
		#[clap(short, long, value_parser)]
		key: String,

		/// The version of the key
		#[clap(short = 'i', long, value_parser)]
		version_id: String,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
	},

}

#[derive(Debug, Clone, ValueEnum)]
pub enum ListEntrySortOrder {
	Ascending,
	Descending,
}

#[derive(Debug, Args)]
pub struct DataStore {
	#[clap(subcommand)]
	command: DataStoreCommands,
}

impl DataStore {
	pub async fn run(self) -> anyhow::Result<Option<String>> {
		match self.command {
			DataStoreCommands::ListStores { prefix, limit, cursor, universe_id, api_key } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let datastore = rbx_cloud.datastore();
				let res = datastore.list_stores(prefix, limit, cursor).await;
				match res {
					Ok(data) => {
						Ok(Some(format!("{:#?}", data)))
					}
					Err(err) => {
						Err(err)
					}
				}
			},

			DataStoreCommands::List { prefix, limit, cursor, universe_id, api_key, datastore_name, scope, all_scopes } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let datastore = rbx_cloud.datastore();
				let res = datastore.list_entries(datastore_name, scope, all_scopes, prefix, limit, cursor).await;
				match res {
					Ok(data) => {
						Ok(Some(format!("{:#?}", data)))
					}
					Err(err) => {
						Err(err)
					}
				}
			},

			DataStoreCommands::Get { datastore_name, scope, key, universe_id, api_key } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let datastore = rbx_cloud.datastore();
				let res = datastore.get_entry_string(datastore_name, scope, key).await;
				match res {
					Ok(data) => {
						Ok(Some(data))
					}
					Err(err) => {
						Err(err)
					}
				}
			},

			DataStoreCommands::Set { datastore_name, scope, key, match_version, exclusive_create, data, user_ids, attributes, universe_id, api_key } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let datastore = rbx_cloud.datastore();
				let res = datastore.set_entry(datastore_name, scope, key, match_version, exclusive_create, user_ids, attributes, data).await;
				match res {
					Ok(data) => {
						Ok(Some(format!("{:#?}", data)))
					}
					Err(err) => {
						Err(err)
					}
				}
			}

			DataStoreCommands::Increment { datastore_name, scope, key, increment_by, user_ids, attributes, universe_id, api_key } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let datastore = rbx_cloud.datastore();
				let res = datastore.increment_entry(datastore_name, scope, key, user_ids, attributes, increment_by).await;
				match res {
					Ok(data) => {
						Ok(Some(format!("{}", data)))
					}
					Err(err) => {
						Err(err)
					}
				}
			}

			DataStoreCommands::Delete { datastore_name, scope, key, universe_id, api_key } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let datastore = rbx_cloud.datastore();
				let res = datastore.delete_entry(datastore_name, scope, key).await;
				match res {
					Ok(_) => {
						Ok(None)
					}
					Err(err) => {
						Err(err)
					}
				}
			}

			DataStoreCommands::ListVersions { datastore_name, scope, key, start_time, end_time, sort_order, limit, cursor, universe_id, api_key } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let datastore = rbx_cloud.datastore();
				let res = datastore.list_entry_versions(datastore_name, scope, key, start_time, end_time, format!("{:?}", sort_order), limit, cursor).await;
				match res {
					Ok(data) => {
						Ok(Some(format!("{:#?}", data)))
					}
					Err(err) => {
						Err(err)
					}
				}
			}

			DataStoreCommands::GetVersion { datastore_name, scope, key, version_id, universe_id, api_key } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let datastore = rbx_cloud.datastore();
				let res = datastore.get_entry_version(datastore_name, scope, key, version_id).await;
				match res {
					Ok(data) => {
						Ok(Some(data))
					}
					Err(err) => {
						Err(err)
					}
				}
			}
		}
	}
}
