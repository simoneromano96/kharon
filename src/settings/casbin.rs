use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CasbinConfig {
  /// Casbin access model path
  pub accessmodelpath: String,
}