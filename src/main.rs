mod graphql;
mod init;
mod settings;
mod types;

use std::sync::Arc;
use tokio::sync::Mutex;

use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::{
	http::{playground_source, GraphQLPlaygroundConfig},
	EmptySubscription,
};
use async_graphql::{Context, Data, EmptyMutation, Object, Schema, Subscription};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use graphql::{MutationRoot, MySchema, QueryRoot};
use init::init_casbin;
use types::AppContext;

use crate::settings::APP_SETTINGS;

struct MyToken(String);

/*
struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
		async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
				if ctx.data::<MyToken>()?.0 != "123456" {
						return Err("Forbidden".into());
				}
				Ok(stream::once(async move { 10 }))
		}
}
*/

async fn index(schema: web::Data<MySchema>, req: HttpRequest, gql_request: Request) -> Response {
	let token = req.headers().get("Token").and_then(|value| value.to_str().map(|s| MyToken(s.to_string())).ok());
	let mut request = gql_request.into_inner();
	if let Some(token) = token {
		request = request.data(token);
	}
	schema.execute(request).await.into()
}

/*
async fn gql_playgound() -> HttpResponse {
		HttpResponse::Ok()
				.content_type("text/html; charset=utf-8")
				.body(playground_source(
						GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
				))
}
*/

/*
async fn index_ws(
		schema: web::Data<MySchema>,
		req: HttpRequest,
		payload: web::Payload,
) -> Result<HttpResponse> {
		WSSubscription::start_with_initializer(Schema::clone(&*schema), &req, payload, |value| async {
				#[derive(serde_derive::Deserialize)]
				struct Payload {
						token: String,
				}

				if let Ok(payload) = serde_json::from_value::<Payload>(value) {
						let mut data = Data::default();
						data.insert(MyToken(payload.token));
						Ok(data)
				} else {
						Err("Token is required".into())
				}
		})
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// let database = init_sqlx().await.expect("Could not initialize DB");

	let enforcer = init_casbin().await.expect("Could not init casbin");

	let enforcer = Arc::new(Mutex::new(enforcer));

	let app_context = AppContext { enforcer };

	let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription).data(app_context).finish();

	HttpServer::new(move || {
		App::new().data(schema.clone()).service(web::resource("/graphql").guard(guard::Post()).to(index))
		// .service(
		//     web::resource("/")
		//         .guard(guard::Get())
		//         .guard(guard::Header("upgrade", "websocket"))
		//         .to(index_ws),
		// )
		// .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
	})
	.bind(format!("0.0.0.0:{}", APP_SETTINGS.app.port))?
	.run()
	.await
}
