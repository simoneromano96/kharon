use casbinrs_mongo_adapter::{
	casbin::{CoreApi, DefaultModel, Enforcer, Result},
	MongoAdapter,
};

use crate::settings::APP_SETTINGS;

pub async fn init_casbin() -> Result<Enforcer> {
	let m = DefaultModel::from_file(&APP_SETTINGS.casbin.accessmodelpath).await?;
	let a = MongoAdapter::new(&APP_SETTINGS.database.uri).await?;
	let e = Enforcer::new(m, a).await?;

	Ok(e)
}
