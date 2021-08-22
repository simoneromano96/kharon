use sqlx_adapter::casbin::{CoreApi, Enforcer, Error as CasbinError, MgmtApi, RbacApi};
use tokio::sync::MutexGuard;

use crate::authorization::AddPermissionForUserInput;

use super::{AddPermissionsForUserInput, AddRolesForUserInput, PermissionInput, UserRoleInput, UserRolesInput, UsersForRoleInput};

/// Asks if a user has a permission
pub fn has_permission_for_user(input: PermissionInput, e: MutexGuard<Enforcer>) -> bool {
	let PermissionInput {
		subject,
		domain,
		action,
		object,
	} = input;
	let authorized = e.has_permission_for_user(&subject, vec![domain, object, action]);
	authorized
}

/// Ask if someone has a permission, this enforces a generic policy
pub fn enforce_policy(input: PermissionInput, e: MutexGuard<Enforcer>) -> Result<bool, CasbinError> {
	let PermissionInput {
		subject,
		domain,
		action,
		object,
	} = input;
	let authorized = e.enforce((subject, domain, object, action))?;
	Ok(authorized)
}

/// Asks all the roles of a user in the given domain.
pub fn get_roles_for_user(input: UserRolesInput, mut e: MutexGuard<Enforcer>) -> Vec<String> {
	let UserRolesInput { subject, domain } = input;
	let roles = e.get_roles_for_user(&subject, Some(&domain));
	roles
}

/// Get all the users of a role in the given domain.
pub fn get_users_for_role(input: UsersForRoleInput, e: MutexGuard<Enforcer>) -> Vec<String> {
	let UsersForRoleInput { role, domain } = input;
	let users = e.get_users_for_role(&role, Some(&domain));
	users
}

/// Asks if a user has a role in the given domain.
pub fn has_role_for_user(input: UserRoleInput, mut e: MutexGuard<Enforcer>) -> bool {
	let UserRoleInput { subject, role, domain } = input;
	let has_role = e.has_role_for_user(&subject, &role, Some(&domain));
	has_role
}

/// Add a specific policy, the subject is either a role or a user.
pub async fn add_policy(input: PermissionInput, mut e: MutexGuard<'_, Enforcer>) -> Result<bool, CasbinError> {
	let PermissionInput {
		subject,
		domain,
		action,
		object,
	} = input;
	let added = e.add_policy(vec![subject, domain, object, action]).await?;
	Ok(added)
}

/// Adds a role to a user in the given domain.
pub async fn add_role_for_user(input: UserRoleInput, mut e: MutexGuard<'_, Enforcer>) -> Result<bool, CasbinError> {
	let UserRoleInput { subject, role, domain } = input;
	let added = e.add_role_for_user(&subject, &role, Some(&domain)).await?;
	Ok(added)
}

/// Adds multiple roles to a user in the given domain.
pub async fn add_roles_for_user(input: AddRolesForUserInput, mut e: MutexGuard<'_, Enforcer>) -> Result<bool, CasbinError> {
	let AddRolesForUserInput { subject, roles, domain } = input;
	let all_added = e.add_roles_for_user(&subject, roles, Some(&domain)).await?;
	Ok(all_added)
}

/// Add a permission to a user.
pub async fn add_permission_for_user(input: AddPermissionForUserInput, mut e: MutexGuard<'_, Enforcer>) -> Result<bool, CasbinError> {
	let AddPermissionForUserInput { subject, permission } = input;
	let added = e.add_permission_for_user(&subject, vec![permission]).await?;
	Ok(added)
}

/// Adds multiple permissions to a user.
pub async fn add_permissions_for_user(input: AddPermissionsForUserInput, mut e: MutexGuard<'_, Enforcer>) -> Result<bool, CasbinError> {
	let AddPermissionsForUserInput { subject, permissions } = input;
	let added = e.add_permissions_for_user(&subject, permissions).await?;
	Ok(added)
}

/// Deletes a role from a user in the given domain.
pub async fn delete_role_for_user(input: UserRoleInput, mut e: MutexGuard<'_, Enforcer>) -> Result<bool, CasbinError> {
	let UserRoleInput { subject, role, domain } = input;
	let added = e.delete_role_for_user(&subject, &role, Some(&domain)).await?;
	Ok(added)
}

/// Deletes multiple roles from a user in the given domain.
pub async fn delete_all_roles_for_user(input: UserRolesInput, mut e: MutexGuard<'_, Enforcer>) -> Result<bool, CasbinError> {
	let UserRolesInput { subject, domain } = input;
	let all_added = e.delete_roles_for_user(&subject, Some(&domain)).await?;
	Ok(all_added)
}
