use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    OrderedDataStoreCreateEntry, OrderedDataStoreEntry, OrderedDataStoreIncrementEntry,
    OrderedDataStoreListEntries, OrderedDataStoreUpdateEntry, RbxCloud, UniverseId,
};

#[derive(Debug, Subcommand)]
pub enum OrderedDataStoreCommands {
    /// List entries
    List {
        /// DataStore name
        #[clap(long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// Maximum number of items to return per page
        #[clap(short, long, value_parser)]
        max_page_size: Option<u64>,

        /// Cursor for the next set of data
        #[clap(short, long, value_parser)]
        page_token: Option<String>,

        /// The enumeration direction (Use 'desc' for descending)
        #[clap(short, long, value_parser)]
        order_by: Option<String>,

        /// A range of qualifying values of entries to return
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Create or overwrite an entry
    Create {
        /// DataStore name
        #[clap(long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        /// The value of the entry
        #[clap(short, long, value_parser)]
        value: i64,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get an entry
    Get {
        /// DataStore name
        #[clap(long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Delete an entry
    Delete {
        /// DataStore name
        #[clap(long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update an entry
    Update {
        /// DataStore name
        #[clap(long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        /// The value of the entry
        #[clap(short, long, value_parser)]
        value: i64,

        /// Create if missing
        #[clap(short = 'm', long, value_parser)]
        allow_missing: Option<bool>,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Increment an entry
    Increment {
        /// DataStore name
        #[clap(long, value_parser)]
        datastore_name: String,

        /// DataStore scope
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        /// The incremented value of the entry
        #[clap(short = 'n', long, value_parser)]
        increment: i64,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct OrderedDataStore {
    #[clap(subcommand)]
    command: OrderedDataStoreCommands,
}

impl OrderedDataStore {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            OrderedDataStoreCommands::List {
                datastore_name,
                scope,
                max_page_size,
                page_token,
                order_by,
                filter,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key, UniverseId(universe_id));
                let ordered_datastore = rbx_cloud.ordered_datastore();
                let res = ordered_datastore
                    .list_entries(&OrderedDataStoreListEntries {
                        name: datastore_name,
                        scope,
                        max_page_size: max_page_size.map(|p| p.into()),
                        page_token,
                        order_by,
                        filter,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            OrderedDataStoreCommands::Create {
                datastore_name,
                scope,
                id,
                value,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key, UniverseId(universe_id));
                let ordered_datastore = rbx_cloud.ordered_datastore();
                let res = ordered_datastore
                    .create_entry(&OrderedDataStoreCreateEntry {
                        name: datastore_name,
                        scope,
                        id,
                        value,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            OrderedDataStoreCommands::Get {
                datastore_name,
                scope,
                id,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key, UniverseId(universe_id));
                let ordered_datastore = rbx_cloud.ordered_datastore();
                let res = ordered_datastore
                    .get_entry(&OrderedDataStoreEntry {
                        name: datastore_name,
                        scope,
                        id,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            OrderedDataStoreCommands::Delete {
                datastore_name,
                scope,
                id,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key, UniverseId(universe_id));
                let ordered_datastore = rbx_cloud.ordered_datastore();
                let res = ordered_datastore
                    .delete_entry(&OrderedDataStoreEntry {
                        name: datastore_name,
                        scope,
                        id,
                    })
                    .await;
                match res {
                    Ok(_) => Ok(None),
                    Err(err) => Err(err.into()),
                }
            }

            OrderedDataStoreCommands::Update {
                datastore_name,
                scope,
                id,
                value,
                allow_missing,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key, UniverseId(universe_id));
                let ordered_datastore = rbx_cloud.ordered_datastore();
                let res = ordered_datastore
                    .update_entry(&OrderedDataStoreUpdateEntry {
                        name: datastore_name,
                        scope,
                        id,
                        value,
                        allow_missing,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }

            OrderedDataStoreCommands::Increment {
                datastore_name,
                scope,
                id,
                increment,
                universe_id,
                api_key,
            } => {
                let rbx_cloud = RbxCloud::new(&api_key, UniverseId(universe_id));
                let ordered_datastore = rbx_cloud.ordered_datastore();
                let res = ordered_datastore
                    .increment_entry(&OrderedDataStoreIncrementEntry {
                        name: datastore_name,
                        scope,
                        id,
                        increment,
                    })
                    .await;
                match res {
                    Ok(data) => Ok(Some(format!("{data:#?}"))),
                    Err(err) => Err(err.into()),
                }
            }
        }
    }
}
