// use casbinrs_mongo_adapter::{
// 	casbin::{CoreApi, DefaultModel, Enforcer, Result},
// 	MongoAdapter,
// };
use sqlx_adapter::casbin::prelude::*;
use sqlx_adapter::casbin::Result;
use sqlx_adapter::SqlxAdapter;

use crate::settings::APP_SETTINGS;

pub async fn init_casbin() -> Result<Enforcer> {
	// let m = DefaultModel::from_file(&APP_SETTINGS.casbin.accessmodelpath).await?;
	// let a = MongoAdapter::new(&APP_SETTINGS.database.uri).await?;
	// let e = Enforcer::new(m, a).await?;

	println!("{:?}", &APP_SETTINGS.database.uri);

	let m = DefaultModel::from_file(&APP_SETTINGS.casbin.accessmodelpath).await?;
	let a = SqlxAdapter::new(&APP_SETTINGS.database.uri, 8).await?;
	let e = Enforcer::new(m, a).await?;

	Ok(e)
}
