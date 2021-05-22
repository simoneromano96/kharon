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
  
  /// Ask all the roles of a user
  async fn get_roles_for_user(&self, ctx: &Context<'_>, user: String, domain: String) -> Result<Vec<String>> {
    let AppContext { enforcer } = ctx.data()?;

    let mut e = enforcer.lock().await;
    let roles = e.get_roles_for_user(&user, Some(&domain));

    Ok(roles)
  }

  /// Ask all the users of a role
  async fn get_users_for_role(&self, ctx: &Context<'_>, role: String, domain: String) -> Result<Vec<String>> {
    let AppContext { enforcer } = ctx.data()?;

    let e = enforcer.lock().await;
    let users = e.get_users_for_role(&role, Some(&domain));

    Ok(users)
  }

  /// Ask if a user has a role
  async fn has_role_for_user(&self, ctx: &Context<'_>, user: String, role: String, domain: String) -> Result<bool> {
    let AppContext { enforcer } = ctx.data()?;

    let mut e = enforcer.lock().await;
    let has_role = e.has_role_for_user(&user, &role, Some(&domain));

    Ok(has_role)
  }
}
