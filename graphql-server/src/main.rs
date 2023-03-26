use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use tokio::signal;

use database_manager::QueryRoot;

#[tokio::main]
async fn main() {
    let dbm = database_manager::init_db().await;

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(dbm)
        .finish();


        let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    tokio::select! {
        _ = ctrl_c => {}
    }

    // Does the db connection need to be closed here?
    // This should drop the schema object that owns the db manager,
    // and therefore the connection pool. I would assume connections
    // are closed when the connection pool is dropped,
    // but there are reports of sqlx not closing sqlite connections
    // when dropped. I'll need to investigate further
    println!("signal received, starting graceful shutdown");
}

