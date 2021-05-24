use anyhow::Result;
use wither::{
	self,
	mongodb::{Client as MongoClient, Database},
	Model,
};

use crate::{auth::AuthClient, settings::APP_SETTINGS};

pub async fn init_database() -> Result<Database> {
	let db = MongoClient::with_uri_str(&APP_SETTINGS.database.uri)
		.await?
		.database(&APP_SETTINGS.database.name);

	AuthClient::sync(&db).await?;

	Ok(db)
}
