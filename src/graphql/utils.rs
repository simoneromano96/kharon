use async_graphql::{Context, Error as AsyncGraphQLError};
use sqlx_adapter::casbin::Enforcer;
use tokio::sync::MutexGuard;

use crate::types::AppContext;

pub async fn get_enforcer_from_context<'a>(ctx: &'a Context<'_>) -> Result<MutexGuard<'a, Enforcer>, AsyncGraphQLError> {
	let AppContext { enforcer, .. } = ctx.data()?;
	let e = enforcer.lock().await;
	Ok(e)
}
