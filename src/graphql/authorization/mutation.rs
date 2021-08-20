use async_graphql::{Context, Object, Result, guard::Guard};
use sqlx_adapter::casbin::{MgmtApi, RbacApi};

use crate::{graphql::{authorization::UserRolesInput, client_guard::ClientGuard}, types::AppContext};

use super::PermissionInput;

#[derive(Default)]
pub struct AuthorizationMutation;

#[Object]
impl AuthorizationMutation {
	/// Create a permission
	///
	/// In the RBAC API the subject is the role, in the ABAC API the subject is the user
	#[graphql(guard(ClientGuard()))]
	async fn create_permission(&self, ctx: &Context<'_>, input: PermissionInput) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;

		let PermissionInput {
			subject,
			domain,
			action,
			object,
		} = input;
		let mut e = enforcer.lock().await;

		let added = e.add_policy(vec![subject, domain, object, action]).await?;

		Ok(String::from(format!("Added: {:?}", added)))
	}

	/// Add role to a user
	#[graphql(guard(ClientGuard()))]
	async fn add_role_for_user(&self, ctx: &Context<'_>, user: String, role: String, domain: String) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let mut e = enforcer.lock().await;

		let added = e.add_role_for_user(&user, &role, Some(&domain)).await?;

		Ok(String::from(format!("Added: {:?}", added)))
	}

	/// Add roles to a user
	#[graphql(guard(ClientGuard()))]
	async fn add_roles_for_user(&self, ctx: &Context<'_>, user: String, roles: Vec<String>, domain: String) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let mut e = enforcer.lock().await;

		let all_added = e.add_roles_for_user(&user, roles, Some(&domain)).await?;

		Ok(String::from(format!("Added: {:?}", all_added)))
	}

	/// Add a permission to a user
	#[graphql(guard(ClientGuard()))]
	async fn add_permission_for_user(&self, ctx: &Context<'_>, user: String, permission: String) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let mut e = enforcer.lock().await;

		let added = e.add_permission_for_user(&user, vec![permission]).await?;

		Ok(String::from(format!("Added: {:?}", added)))
	}

	/// Add permissions to a user
	#[graphql(guard(ClientGuard()))]
	async fn add_permissions_for_user(&self, ctx: &Context<'_>, user: String, permissions: Vec<Vec<String>>) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let mut e = enforcer.lock().await;

		let added = e.add_permissions_for_user(&user, permissions).await?;

		Ok(String::from(format!("Added: {:?}", added)))
	}

	/// Delete role from a user
	#[graphql(guard(ClientGuard()))]
	async fn delete_role_for_user(&self, ctx: &Context<'_>, user: String, role: String, domain: String) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let mut e = enforcer.lock().await;

		let added = e.delete_role_for_user(&user, &role, Some(&domain)).await?;

		Ok(String::from(format!("Deleted: {:?}", added)))
	}

	/// Delete all roles from a user
	#[graphql(guard(ClientGuard()))]
	async fn delete_all_roles_for_user(&self, ctx: &Context<'_>, input: UserRolesInput) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let UserRolesInput { subject, domain } = input;

		let mut e = enforcer.lock().await;

		let all_added = e.delete_roles_for_user(&subject, Some(&domain)).await?;

		Ok(String::from(format!("Deleted: {:?}", all_added)))
	}

	/// Delete a user
	#[graphql(guard(ClientGuard()))]
	async fn delete_user(&self, ctx: &Context<'_>, user: String) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let mut e = enforcer.lock().await;

		let deleted = e.delete_user(&user).await?;

		Ok(String::from(format!("Deleted: {:?}", deleted)))
	}

	/// Delete a role
	#[graphql(guard(ClientGuard()))]
	async fn delete_role(&self, ctx: &Context<'_>, role: String) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let mut e = enforcer.lock().await;

		let deleted = e.delete_role(&role).await?;

		Ok(String::from(format!("Deleted: {:?}", deleted)))
	}

	/// Delete a permission
	#[graphql(guard(ClientGuard()))]
	async fn delete_permission(&self, ctx: &Context<'_>, permission: String) -> Result<String> {
		let AppContext { enforcer, .. } = ctx.data()?;
		let mut e = enforcer.lock().await;

		let deleted = e.delete_permission(vec![permission]).await?;

		Ok(String::from(format!("Deleted: {:?}", deleted)))
	}
}
