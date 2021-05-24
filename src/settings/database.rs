use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// SQL database configuration
pub struct DatabaseConfig {
	/// DB Connection URI
	pub uri: String,
	/// Name of the database
	pub name: String,
}
