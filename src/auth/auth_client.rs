use serde::{Deserialize, Serialize};
use wither::{
	bson::{doc, oid::ObjectId},
	prelude::*,
};

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[model(index(keys = r#"doc!{"name": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct AuthClient {
	/// The client ID
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	/// The client unique username
	pub name: String,
	/// The client password
	pub password: String,
}
