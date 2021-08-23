use std::borrow::{Borrow, Cow};

use actix_web_httpauth::headers::authorization::Basic;
use ormx::conditional_query_as;
use sqlx::{Pool, Postgres, Error as SqlxError};
use thiserror::Error;
// use wither::{bson::doc, mongodb::Database, Model, WitherError};

use super::AuthClient;

#[derive(Error, Debug)]
pub enum ClientAuthenticationErrors {
	#[error("{0}")]
	DatabaseError(#[from] SqlxError),
	#[error("Could not find client")]
	ClientNotFound,
}

pub async fn authenticate_client(database: &Pool<Postgres>, scheme: Basic) -> Result<AuthClient, ClientAuthenticationErrors> {
	let name: &str = scheme.user_id().borrow();
	let password: &str = scheme.password().unwrap_or(&Cow::Borrowed("")).borrow();

	// let auth_client = AuthClient::find_one(database, doc! { "name": name, "password": password }, None).await?;
  let auth_client = conditional_query_as!(
    AuthClient,
    "SELECT * FROM auth_client"
    "WHERE name =" ?(name)
    "AND password =" ?(password)
  ).fetch_optional(database).await?;

	match auth_client {
    Some(auth_client) => Ok(auth_client),
    None => Err(ClientAuthenticationErrors::ClientNotFound),
  }
}
