use sqlx_adapter::{SqlxAdapter, casbin::{CoreApi, DefaultModel, Enforcer, Result}};

use crate::settings::APP_SETTINGS;

pub async fn init_casbin() -> Result<Enforcer> {
  let m = DefaultModel::from_file(&APP_SETTINGS.casbin.accessmodelpath).await?;
  let a = SqlxAdapter::new(&APP_SETTINGS.database.uri, 10).await?;
  let e = Enforcer::new(m, a).await?;

  Ok(e)
}
