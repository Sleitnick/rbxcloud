use clap::{Args, Subcommand, ValueEnum};

use rbxcloud::rbx::{
    types::{ReturnLimit, RobloxUserId, UniverseId},
    v1::{
        DataStoreDeleteEntry, DataStoreGetEntry, DataStoreGetEntryVersion, DataStoreIncrementEntry,
        DataStoreListEntries, DataStoreListEntryVersions, DataStoreListStores, DataStoreSetEntry,
        RbxCloud,
    },
};

#[derive(Debug, Subcommand)]
pub enum DataStoreCommands {
    /// List all DataStores in a given universe
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
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// List all entries in a DataStore
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
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get a DataStore entry
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
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Set or create the value of a DataStore entry
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
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Increment or create the value of a DataStore entry
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
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Delete a DataStore entry
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
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// List all versions of a DataStore entry
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
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get the value of a specific entry version
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
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
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

#[inline]
fn u64_ids_to_roblox_ids(user_ids: Option<Vec<u64>>) -> Option<Vec<RobloxUserId>> {
    user_ids.map(|ids| {
        ids.into_iter()
            .map(RobloxUserId)
            .collect::<Vec<RobloxUserId>>()
    })
}

impl DataStore {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            DataStoreCommands::ListStores {
                prefix,
                limit,
                cursor,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let datastore = rbx_cloud.datastore(UniverseId(universe_id));
                let res = datastore
                    .list_stores(&DataStoreListStores {
                        cursor,
                        limit: ReturnLimit(limit),
                        prefix,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::List {
                prefix,
                limit,
                cursor,
                universe_id,
                api_key,
                datastore_name,
                scope,
                all_scopes,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let datastore = rbx_cloud.datastore(UniverseId(universe_id));
                let res = datastore
                    .list_entries(&DataStoreListEntries {
                        name: datastore_name,
                        scope,
                        all_scopes,
                        prefix,
                        limit: ReturnLimit(limit),
                        cursor,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::Get {
                datastore_name,
                scope,
                key,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let datastore = rbx_cloud.datastore(UniverseId(universe_id));
                let res = datastore
                    .get_entry_string(&DataStoreGetEntry {
                        name: datastore_name,
                        scope,
                        key,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(data)),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::Set {
                datastore_name,
                scope,
                key,
                match_version,
                exclusive_create,
                data,
                user_ids,
                attributes,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let datastore = rbx_cloud.datastore(UniverseId(universe_id));
                let ids = u64_ids_to_roblox_ids(user_ids);
                let res = datastore
                    .set_entry(&DataStoreSetEntry {
                        name: datastore_name,
                        scope,
                        key,
                        match_version,
                        exclusive_create,
                        roblox_entry_user_ids: ids,
                        roblox_entry_attributes: attributes,
                        data,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::Increment {
                datastore_name,
                scope,
                key,
                increment_by,
                user_ids,
                attributes,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let datastore = rbx_cloud.datastore(UniverseId(universe_id));
                let ids = u64_ids_to_roblox_ids(user_ids);
                let res = datastore
                    .increment_entry(&DataStoreIncrementEntry {
                        name: datastore_name,
                        scope,
                        key,
                        roblox_entry_user_ids: ids,
                        roblox_entry_attributes: attributes,
                        increment_by,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data}"))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::Delete {
                datastore_name,
                scope,
                key,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let datastore = rbx_cloud.datastore(UniverseId(universe_id));
                let res = datastore
                    .delete_entry(&DataStoreDeleteEntry {
                        name: datastore_name,
                        scope,
                        key,
                    })
                    .await;
                match res {
                    Ok(_) => Ok(None),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::ListVersions {
                datastore_name,
                scope,
                key,
                start_time,
                end_time,
                sort_order,
                limit,
                cursor,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let datastore = rbx_cloud.datastore(UniverseId(universe_id));
                let res = datastore
                    .list_entry_versions(&DataStoreListEntryVersions {
                        name: datastore_name,
                        scope,
                        key,
                        start_time,
                        end_time,
                        sort_order: format!("{sort_order:?}"),
                        limit: ReturnLimit(limit),
                        cursor,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            DataStoreCommands::GetVersion {
                datastore_name,
                scope,
                key,
                version_id,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key);
                let datastore = rbx_cloud.datastore(UniverseId(universe_id));
                let res = datastore
                    .get_entry_version(&DataStoreGetEntryVersion {
                        name: datastore_name,
                        scope,
                        key,
                        version_id,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(data)),
                    Err(err) => Err(err.into()),
                }
            }
        }
    }
}
