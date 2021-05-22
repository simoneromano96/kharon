use std::sync::{Arc};
use tokio::sync::Mutex;

use sqlx_adapter::casbin::Enforcer;

/// App context
pub struct AppContext {
  /// Casbin policy enforcer
  pub enforcer: Arc<Mutex<Enforcer>>,
}
