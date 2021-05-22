use async_graphql::{Context, Object, Result};
use sqlx_adapter::casbin::MgmtApi;

use crate::types::AppContext;

use super::PermissionInput;

#[derive(Default)]
pub struct AuthorizationMutation;

#[Object]
impl AuthorizationMutation {
	/// Create a permission
	async fn create_permission(&self, ctx: &Context<'_>, input: PermissionInput) -> Result<String> {
		let AppContext { enforcer } = ctx.data()?;

		let sub = &input.subject;
		let obj = &input.object;
		let act = &input.action;
		let mut e = enforcer.lock().await;

		let added = e
			.add_policy(vec![sub.clone(), obj.clone(), act.clone()])
			.await
			.expect("Cannot add policy");

		Ok(String::from(format!("Added: {:?}", added)))
	}
}
