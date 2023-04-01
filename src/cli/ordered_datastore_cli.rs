use clap::{Args, Subcommand};
use rbxcloud::rbx::{OrderedDataStoreListEntries, RbxCloud, UniverseId};

#[derive(Debug, Subcommand)]
pub enum OrderedDataStoreCommands {
    /// List all entries in a DataStore
    List {
        /// DataStore name
        #[clap(short, long, value_parser)]
        ordered_datastore_name: String,

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
                ordered_datastore_name,
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
                        name: ordered_datastore_name,
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
        }
    }
}