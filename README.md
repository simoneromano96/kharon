# kharon

Kharon is an GQL ACPM service, takes inspiration from my previous work.

This service is made for a Backend to Backend architecture, so it uses Basic Auth.

The passwords of the clients are not hashed.

## Todos

* [ ] Add REST interface

* [ ] Add Paseto token support

## Environment

* `APP_DATABASE_URI`: the postgres database URI

* `APP_CASBIN_ACCESSMODELPATH`: the access model path for Casbin, default supports RBAC/ABAC with domains

* `APP_SERVER_PORT`: the port for the server to listen at

## Example usage

Basic RBAC usage with domains:

```graphql
# `admin` (role) can `read` `products` in the `shop1` domain
mutation addPolicy{
  addPolicy(input: {subject: "admin", action: "read", object: "products" domain: "shop1"})
}

# `user1` (user) is an `admin` in the `shop1` domain
mutation addRoleForUser{
  addRoleForUser(input: {subject: "user1", role: "admin", domain: "shop1"})
}

# Can `user1` `read` `products` of `shop1`?
query hasPermission{
  hasPermission(input: {subject: "user1", action: "read", object: "products", domain: "shop1"})
}
```
