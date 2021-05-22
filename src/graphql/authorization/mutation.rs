use async_graphql::{Context, Object, Result};
use sqlx_adapter::casbin::{MgmtApi, RbacApi};

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

	/// Add role to a user
	async fn add_role_for_user(&self, ctx: &Context<'_>, user: String, role: String, domain: String) -> Result<String> {
		let AppContext { enforcer } = ctx.data()?;
		let mut e = enforcer.lock().await;

    let added = e.add_role_for_user(&user, &role, Some(&domain)).await?;
    
		Ok(String::from(format!("Added: {:?}", added)))
	}

	/// Add roles to a user
	async fn add_roles_for_user(&self, ctx: &Context<'_>, user: String, roles: Vec<String>, domain: String) -> Result<String> {
		let AppContext { enforcer } = ctx.data()?;
		let mut e = enforcer.lock().await;

    let all_added = e.add_roles_for_user(&user, roles, Some(&domain)).await?;
    
		Ok(String::from(format!("Added: {:?}", all_added)))
	}

	/// Delete role from a user
	async fn delete_role_for_user(&self, ctx: &Context<'_>, user: String, role: String, domain: String) -> Result<String> {
		let AppContext { enforcer } = ctx.data()?;
		let mut e = enforcer.lock().await;

    let added = e.delete_role_for_user(&user, &role, Some(&domain)).await?;
    
		Ok(String::from(format!("Deleted: {:?}", added)))
	}

	/// Delete all roles from a user
	async fn delete_all_roles_for_user(&self, ctx: &Context<'_>, user: String, domain: String) -> Result<String> {
		let AppContext { enforcer } = ctx.data()?;
		let mut e = enforcer.lock().await;

    let all_added = e.delete_roles_for_user(&user, Some(&domain)).await?;
    
		Ok(String::from(format!("Deleted: {:?}", all_added)))
	}
}
