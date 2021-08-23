use async_graphql::{guard::Guard, Context, Error, Object, Result};

use crate::{
	authorization::{
		enforce_policy, get_roles_for_user, get_users_for_role, has_permission_for_user, has_role_for_user, UserRoleInput,
		PermissionInput, UserRolesInput, UsersForRoleInput,
	},
	graphql::{client_guard::ClientGuard, get_enforcer_from_context},
};

#[derive(Default)]
pub struct AuthorizationQuery;

#[Object]
impl AuthorizationQuery {
	/// Ask if someone has a permission
	///
	/// The subject can be either a role or a user.
	#[graphql(cache_control(max_age = 3600), guard(ClientGuard()))]
	async fn has_permission(&self, ctx: &Context<'_>, input: PermissionInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let authorized = has_permission_for_user(input, e);

		if authorized {
			Ok(String::from("Authorized"))
		} else {
			Err(Error::from("Not authorized"))
		}
	}

	/// Ask if someone has a permission
	///
	/// The subject can be either a user or a role
	#[graphql(cache_control(max_age = 3600), guard(ClientGuard()))]
	async fn enforce_permission(&self, ctx: &Context<'_>, input: PermissionInput) -> Result<String> {
		let e = get_enforcer_from_context(ctx).await?;
		let authorized = enforce_policy(input, e)?;

		if authorized {
			Ok(String::from("Authorized"))
		} else {
			Err(Error::from("Not authorized"))
		}
	}

	/// Ask all the roles of a user
	#[graphql(cache_control(max_age = 3600), guard(ClientGuard()))]
	async fn get_roles_for_user(&self, ctx: &Context<'_>, input: UserRolesInput) -> Result<Vec<String>> {
		let e = get_enforcer_from_context(ctx).await?;
		let roles = get_roles_for_user(input, e);

		Ok(roles)
	}

	/// Ask all the users of a role
	#[graphql(cache_control(max_age = 3600), guard(ClientGuard()))]
	async fn get_users_for_role(&self, ctx: &Context<'_>, input: UsersForRoleInput) -> Result<Vec<String>> {
		let e = get_enforcer_from_context(ctx).await?;
		let users = get_users_for_role(input, e);

		Ok(users)
	}

	/// Ask if a user has a role
	#[graphql(cache_control(max_age = 3600), guard(ClientGuard()))]
	async fn has_role_for_user(&self, ctx: &Context<'_>, input: UserRoleInput) -> Result<bool> {
		let e = get_enforcer_from_context(ctx).await?;
		let has_role = has_role_for_user(input, e);

		Ok(has_role)
	}
}
