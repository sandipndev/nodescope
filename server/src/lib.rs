mod config;
pub use config::ServerConfig;

mod graphql;

use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Extension, Router, routing::get};

use app::NodeScopeApp;

pub async fn run(config: ServerConfig, app: NodeScopeApp) -> anyhow::Result<()> {
    let port = config.port;

    let app = Router::new()
        .route("/health", get(health_check))
        .route(
            "/graphql",
            get(playground).post(axum::routing::post(graphql_handler)),
        )
        .layer(Extension(config))
        .layer(Extension(app));

    let listener =
        tokio::net::TcpListener::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], port))).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

pub async fn graphql_handler(
    schema: Extension<Schema<graphql::Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    let response = schema.execute(req).await;
    response.into()
}

async fn playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

async fn health_check() -> &'static str {
    "OK"
}
