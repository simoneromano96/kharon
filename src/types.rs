use std::sync::Arc;
use tokio::sync::Mutex;

use casbinrs_mongo_adapter::casbin::Enforcer;
use wither::mongodb::Database;

/// App context
#[derive(Clone)]
pub struct AppContext {
	/// Casbin policy enforcer
	pub enforcer: Arc<Mutex<Enforcer>>,
	/// Database connection
	pub database: Database,
}
