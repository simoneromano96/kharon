use std::borrow::{Borrow, Cow};

use actix_web_httpauth::headers::authorization::Basic;
use thiserror::Error;
use wither::{bson::doc, mongodb::Database, Model, WitherError};

use super::AuthClient;

#[derive(Error, Debug)]
pub enum ClientAuthenticationErrors {
	#[error("{0}")]
	DatabaseError(#[from] WitherError),
	#[error("Could not find client")]
	ClientNotFound,
}

pub async fn authenticate_client(database: &Database, scheme: Basic) -> Result<AuthClient, ClientAuthenticationErrors> {
	let name: &str = scheme.user_id().borrow();
	let password: &str = scheme.password().unwrap_or(&Cow::Borrowed("")).borrow();

	let auth_client = AuthClient::find_one(database, doc! { "name": name, "password": password }, None).await?;

	match auth_client {
    Some(auth_client) => Ok(auth_client),
    None => Err(ClientAuthenticationErrors::ClientNotFound),
  }
}
