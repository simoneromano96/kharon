use async_graphql::InputObject;

#[derive(InputObject)]
/// Describe the question: "Has the `subject` the permission to do `action` to `object`?".
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

#[derive(InputObject)]
/// Used to describe the roles of a user in a domain.
pub struct UserRolesInput {
	/// The subject (user) trying to do something.
	pub subject: String,
	/// The domain of the subject (user).
	pub domain: String,
}
