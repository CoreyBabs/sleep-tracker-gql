/// Main entry point and managing of the server itself

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use tokio::signal;

use database_manager::{QueryRoot, MutationRoot};

#[tokio::main]
async fn main() {
    // dbm is currently being created with a test db that gets deleted and recreated every startup
    // Uncomment following line for using a real db
    // let dbm = database_manager::init_db().await;
    let dbm = database_manager::_init_test_db().await;

    // Build schema with queries and mutations, then set the database manager as the context
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(dbm)
        .finish();

    // setup the axum app with the schema, and setup the graphiql editor
    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    // Bind server to local host port 8000, provide handler for graceful shutdown
    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Basic and default graphql handler from the axum/async-graphql docs
async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// binds graphiql to default url
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

/// handle graceful shutdown, in this case, ctrl+c is only supported graceful shutdown
/// More options should be added here.
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

