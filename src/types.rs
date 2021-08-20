use std::sync::Arc;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

// use casbinrs_mongo_adapter::casbin::Enforcer;
// use wither::mongodb::Database;
use sqlx_adapter::casbin::Enforcer;

/// App context
#[derive(Clone)]
pub struct AppContext {
	/// Casbin policy enforcer
	pub enforcer: Arc<Mutex<Enforcer>>,
	// Database connection
	pub database: Pool<Postgres>,
	// pub database: Database,
}
