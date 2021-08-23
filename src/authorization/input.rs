use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Serialize, Deserialize)]
/// Describe the question: "Has the `subject` the permission to do `action` to `object`?".
///
/// ```rust
/// let can_alice_read_products_of_ecommerce1 = PermissionInput {
/// 	subject: "alice".to_string(),
/// 	action: "read".to_string(),
/// 	object: "products".to_string(),
/// 	domain: "ecommerce1".to_string(),
/// };
/// ```
///
pub struct PermissionInput {
	/// The subject (user or role) trying to do something.
	pub subject: String,
	/// The action the subject is trying to do.
	pub action: String,
	/// The destination of the action.
	pub object: String,
	/// The domain of the object.
	pub domain: String,
}

#[derive(InputObject, Serialize, Deserialize)]
/// Input used to describe the roles of a user in a domain.
///
/// For example: "what are all the roles of this subject in this domain?" or "delete all the roles of this subject in this domain".
pub struct UserRolesInput {
	/// The subject (user).
	pub subject: String,
	/// The domain of the subject (user).
	pub domain: String,
}

#[derive(InputObject, Serialize, Deserialize)]
/// Input to get all the users of a specified domain and role.
pub struct UsersForRoleInput {
	/// The Role to search.
	pub role: String,
	/// The domain of the role.
	pub domain: String,
}

#[derive(InputObject, Serialize, Deserialize)]
/// Input to ask if a subject (user) has a role, to add a role to a suject (user) or to remove a role from a suject (user).
pub struct UserRoleInput {
	/// The specific user.
	pub subject: String,
	/// The Role to search.
	pub role: String,
	/// The Domain of the role.
	pub domain: String,
}

#[derive(InputObject, Serialize, Deserialize)]
/// Input to add multiple roles to a suject (user).
pub struct AddRolesForUserInput {
	/// The specific user.
	pub subject: String,
	/// The Roles to add.
	pub roles: Vec<String>,
	/// The Domain of the roles.
	pub domain: String,
}

#[derive(InputObject, Serialize, Deserialize)]
/// Input to add a permission to a suject (user).
pub struct AddPermissionForUserInput {
	/// The specific user.
	pub subject: String,
	/// The permission to add.
	pub permission: String,
}

#[derive(InputObject, Serialize, Deserialize)]
/// Input to add permissions to a suject (user).
pub struct AddPermissionsForUserInput {
	/// The specific user.
	pub subject: String,
	/// The permissions to add.
	pub permissions: Vec<Vec<String>>,
}
