mod auth_client;
mod graphql;
mod init;
mod settings;
mod types;
mod authorization;

use std::sync::Arc;
use tokio::sync::Mutex;

use actix_web::{guard, http::header::Header, web, App, HttpRequest, HttpServer};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response};
use graphql::{MutationRoot, MySchema, QueryRoot};
use init::init_casbin;
use web::Data;

use crate::{auth_client::authenticate_client, init::init_sqlx, settings::APP_SETTINGS, types::AppContext};

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

async fn index(schema: Data<MySchema>, app_context: Data<AppContext>, req: HttpRequest, gql_request: Request) -> Response {
	let auth = Authorization::<Basic>::parse(&req);

	let mut request = gql_request.into_inner();
	if let Ok(token) = auth {
		if let Ok(client) = authenticate_client(&app_context.database, token.into_scheme()).await {
			request = request.data(client);
		}
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
	let database = init_sqlx().await.expect("Could not initialize database");

	let enforcer = init_casbin().await.expect("Could not initialize casbin");

	let enforcer = Arc::new(Mutex::new(enforcer));

	// let database = init_database().await.expect("Could not initialize database");

	let app_context = AppContext { enforcer, database };

	let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
		.data(app_context.clone())
		.finish();

	HttpServer::new(move || {
		App::new()
			.app_data(Data::new(schema.clone()))
			.app_data(Data::new(app_context.clone()))
			.service(web::resource("/graphql").guard(guard::Post()).to(index))
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
