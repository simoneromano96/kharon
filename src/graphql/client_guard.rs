use async_graphql::{guard::Guard, Context, Result};
use async_trait::async_trait;
use thiserror::Error;

use crate::auth::AuthClient;

#[derive(Error, Debug)]
pub enum ClientGuardErrors {
	#[error("Unauthenticated client in protected resolver!")]
	UnauthenticatedClient,
}

pub struct ClientGuard {}

#[async_trait]
impl Guard for ClientGuard {
	async fn check(&self, ctx: &Context<'_>) -> Result<()> {
		match ctx.data_opt::<AuthClient>() {
			Some(_) => Ok(()),
			None => Err(async_graphql::Error::from(ClientGuardErrors::UnauthenticatedClient)),
		}
	}
}
