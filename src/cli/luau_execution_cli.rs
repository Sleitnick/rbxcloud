use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::{PlaceId, UniverseId},
    v2::{luau_execution::LuauExecutionTaskLogView, Client},
};
use tokio::fs;

#[derive(Debug, Subcommand)]
pub enum LuauExecutionCommands {
    /// Executes Luau code on Roblox
    Execute {
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID of the experience
        #[clap(short = 'i', long, value_parser)]
        place_id: u64,

        /// Version ID of the experience
        #[clap(short = 'r', long, value_parser)]
        version_id: Option<String>,

        /// Script source code
        #[clap(short, long, value_parser)]
        script: Option<String>,

        /// Script source code file
        #[clap(short, long, value_parser)]
        filepath: Option<String>,

        /// Execution timeout
        #[clap(short, long, value_parser)]
        timeout: Option<String>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Gets information on a previously executed task
    GetTask {
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID of the experience
        #[clap(short = 'i', long, value_parser)]
        place_id: u64,

        /// Version ID of the experience
        #[clap(short = 'r', long, value_parser)]
        version_id: Option<String>,

        /// Luau execution session ID
        #[clap(short, long, value_parser)]
        session_id: String,

        /// Luau execution task ID
        #[clap(short, long, value_parser)]
        task_id: String,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Retrieves logs on a previously executed task
    GetLogs {
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID of the experience
        #[clap(short = 'i', long, value_parser)]
        place_id: u64,

        /// Version ID of the experience
        #[clap(short = 'r', long, value_parser)]
        version_id: Option<String>,

        /// Luau execution session ID
        #[clap(short, long, value_parser)]
        session_id: String,

        /// Luau execution task ID
        #[clap(short, long, value_parser)]
        task_id: String,

        /// Max page size
        #[clap(short, long, value_parser)]
        max_page_size: Option<u32>,

        /// Next page token
        #[clap(short = 'n', long, value_parser)]
        page_token: Option<String>,

        /// Log view type
        #[clap(short = 'w', long, value_enum)]
        view: Option<LuauExecutionTaskLogView>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Luau {
    #[clap(subcommand)]
    command: LuauExecutionCommands,
}

impl Luau {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            LuauExecutionCommands::Execute {
                universe_id,
                place_id,
                version_id,
                script,
                filepath,
                timeout,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);

                let luau = client.luau(UniverseId(universe_id), PlaceId(place_id), version_id);

                let mut script_source: Option<String> = None;
                if script.is_some() {
                    script_source = script;
                }

                if script_source.is_none() {
                    if let Some(path) = filepath {
                        let src_bytes = fs::read(path).await?;
                        script_source = Some(String::from_utf8(src_bytes)?);
                    }
                }

                let src = script_source.expect("script or filepath must be set");

                let res = luau.create_task(src, timeout).await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            LuauExecutionCommands::GetTask {
                universe_id,
                place_id,
                version_id,
                session_id,
                task_id,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);

                let luau = client.luau(UniverseId(universe_id), PlaceId(place_id), version_id);

                let res = luau.get_task(session_id, task_id).await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            LuauExecutionCommands::GetLogs {
                universe_id,
                place_id,
                version_id,
                session_id,
                task_id,
                max_page_size,
                page_token,
                view,
                pretty,
                api_key,
            } => {
                let client = Client::new(&api_key);

                let luau = client.luau(UniverseId(universe_id), PlaceId(place_id), version_id);

                let res = luau
                    .get_logs(
                        session_id,
                        task_id,
                        view.unwrap_or(LuauExecutionTaskLogView::Flat),
                        max_page_size,
                        page_token,
                    )
                    .await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
