pub mod authorization;

use async_graphql::{EmptySubscription, Schema, MergedObject};

use self::authorization::{AuthorizationMutation, AuthorizationQuery};

#[derive(MergedObject, Default)]
pub struct QueryRoot(AuthorizationQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(AuthorizationMutation);

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
