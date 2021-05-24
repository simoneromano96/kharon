use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// App and server configuration
pub struct AppConfig {
	/// Server's port
	pub port: u16,
}
