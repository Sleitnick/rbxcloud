//! # `rbxcloud` - CLI & SDK for Roblox Open Cloud APIs
//!
//! `rbxcloud` is both a CLI and an SDK for the Roblox
//! Open Cloud APIs. For in-depth documentation on
//! both the CLI and SDK functionality, visit the
//! [documentation](https://sleitnick.github.io/rbxcloud/)
//! website.
//!
//! ## Usage
//!
//! Add `rbxcloud` as a dependency.
//! ```sh
//! $ cargo add rbxcloud
//! ```
//!
//! Example: Publishing a message.
//! ```rust,no_run
//! use rbxcloud::rbx::{error::Error, v1::RbxCloud, types::UniverseId};
//!
//! async fn publish_message() -> Result<(), Error> {
//!     let api_key = "my_api_key";
//!     let universe_id = UniverseId(9876543210);
//!     let topic = "MyTopic";
//!     let cloud = RbxCloud::new(api_key);
//!     let messaging = cloud.messaging(universe_id, topic);
//!     messaging.publish("Hello world").await
//! }
//! ```
//!
//! Consuming the message from a Roblox script.
//! ```lua
//! local MessagingService = game:GetService("MessagingService")
//! MessagingService:SubscribeAsync("MyTopic"):Connect(function(payload)
//!     print(payload)
//!     --> {"message": "Hello world"}
//! end)
//! ```
pub mod rbx;
