use async_graphql::{guard::Guard, Context, Object, Result};
use sqlx_adapter::casbin::RbacApi;

use crate::{
	authorization::{
		add_permission_for_user, add_permissions_for_user, add_policy, add_role_for_user, add_roles_for_user, delete_all_roles_for_user,
		delete_role_for_user, AddPermissionForUserInput, AddPermissionsForUserInput, AddRolesForUserInput, PermissionInput, UserRoleInput,
		UserRolesInput,
	},
	graphql::{client_guard::ClientGuard, get_enforcer_from_context},
};

#[derive(Default)]
pub struct AuthorizationMutation;

#[Object]
impl AuthorizationMutation {
	/// Create a permission
	///
	/// In the RBAC API the subject is the role, in the ABAC API the subject is the user
	#[graphql(guard(ClientGuard()))]
	async fn add_policy(&self, ctx: &Context<'_>, input: PermissionInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let added = add_policy(input, e).await?;

		Ok(format!("Added: {:?}", added))
	}

	/// Add role to a user
	#[graphql(guard(ClientGuard()))]
	async fn add_role_for_user(&self, ctx: &Context<'_>, input: UserRoleInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let added = add_role_for_user(input, e).await?;

		Ok(format!("Added: {:?}", added))
	}

	/// Add roles to a user
	#[graphql(guard(ClientGuard()))]
	async fn add_roles_for_user(&self, ctx: &Context<'_>, input: AddRolesForUserInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let all_added = add_roles_for_user(input, e).await?;

		Ok(format!("Added: {:?}", all_added))
	}

	/// Add a permission to a user
	#[graphql(guard(ClientGuard()))]
	async fn add_permission_for_user(&self, ctx: &Context<'_>, input: AddPermissionForUserInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let added = add_permission_for_user(input, e).await?;

		Ok(format!("Added: {:?}", added))
	}

	/// Add permissions to a user
	#[graphql(guard(ClientGuard()))]
	async fn add_permissions_for_user(&self, ctx: &Context<'_>, input: AddPermissionsForUserInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let added = add_permissions_for_user(input, e).await?;

		Ok(format!("Added: {:?}", added))
	}

	/// Delete role from a user
	#[graphql(guard(ClientGuard()))]
	async fn delete_role_for_user(&self, ctx: &Context<'_>, input: UserRoleInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let added = delete_role_for_user(input, e).await?;

		Ok(format!("Deleted: {:?}", added))
	}

	/// Delete all roles from a user
	#[graphql(guard(ClientGuard()))]
	async fn delete_all_roles_for_user(&self, ctx: &Context<'_>, input: UserRolesInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let all_added = delete_all_roles_for_user(input, e).await?;

		Ok(format!("Deleted: {:?}", all_added))
	}

	/// Delete a user
	#[graphql(guard(ClientGuard()))]
	async fn delete_user(&self, ctx: &Context<'_>, user: String) -> Result<String> {
		let mut e = get_enforcer_from_context(ctx).await?;

		let deleted = e.delete_user(&user).await?;

		Ok(format!("Deleted: {:?}", deleted))
	}

	/// Delete a role
	#[graphql(guard(ClientGuard()))]
	async fn delete_role(&self, ctx: &Context<'_>, role: String) -> Result<String> {
		let mut e = get_enforcer_from_context(ctx).await?;

		let deleted = e.delete_role(&role).await?;

		Ok(format!("Deleted: {:?}", deleted))
	}

	/// Delete a permission
	#[graphql(guard(ClientGuard()))]
	async fn delete_permission(&self, ctx: &Context<'_>, permission: String) -> Result<String> {
		let mut e = get_enforcer_from_context(ctx).await?;

		let deleted = e.delete_permission(vec![permission]).await?;

		Ok(format!("Deleted: {:?}", deleted))
	}
}
