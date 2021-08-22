pub mod authorization;
pub mod client_guard;
pub mod utils;

pub use utils::*;

use async_graphql::{EmptySubscription, MergedObject, Schema};

use self::authorization::{AuthorizationMutation, AuthorizationQuery};

#[derive(MergedObject, Default)]
pub struct QueryRoot(AuthorizationQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(AuthorizationMutation);

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
