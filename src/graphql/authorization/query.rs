use async_graphql::{Context, ID, Object, Result};
use sqlx_adapter::casbin::RbacApi;

use crate::{AppContext};

use super::{PermissionInput};

#[derive(Default)]
pub struct AuthorizationQuery;

#[Object]
impl AuthorizationQuery {
  /// Ask if someone has a permission
  async fn has_permission(&self, ctx: &Context<'_>, input: PermissionInput) -> Result<String> {
    let AppContext { enforcer } = ctx.data()?;

    let sub = &input.subject;
    let obj = &input.object;
    let act = &input.action;
    let e = enforcer.lock().await;

    let authorized = e.has_permission_for_user(sub, vec![obj.clone(), act.clone()]);

    Ok(String::from(
      if authorized {
        "Authorized"
      } else {
        "Not authorized"
      }
    ))
  }
}
