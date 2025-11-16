mod config;
pub use config::ServerConfig;

mod graphql;

use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Extension, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get,
};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use app::NodeScopeApp;

pub async fn run(config: ServerConfig, app: NodeScopeApp) -> anyhow::Result<()> {
    let port = config.port;

    let schema = graphql::schema(Some(app.clone()));

    let app = Router::new()
        .route("/health", get(health_check))
        .route(
            "/graphql",
            get(playground).post(axum::routing::post(graphql_handler)),
        )
        .route("/", get(index_handler))
        .route("/{*path}", get(static_handler))
        .layer(Extension(schema))
        .layer(Extension(config))
        .layer(Extension(app));

    let listener =
        tokio::net::TcpListener::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], port))).await?;

    println!("UI and GraphQL server running on port {}", port);
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

#[derive(RustEmbed)]
#[folder = "../dashboard/dist"]
struct Assets;

async fn index_handler() -> impl IntoResponse {
    if let Some(content) = Assets::get("index.html") {
        let mime = from_path("index.html").first_or_octet_stream();
        return (
            [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
            content.data.into_owned(),
        )
            .into_response();
    }
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

pub async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    if let Some(content) = Assets::get(&path) {
        let mime = from_path(&path).first_or_octet_stream();
        return (
            [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
            content.data.into_owned(),
        )
            .into_response();
    }

    // For Vue Router: if file not found, serve index.html
    // This allows client-side routing to work
    if let Some(content) = Assets::get("index.html") {
        let mime = from_path("index.html").first_or_octet_stream();
        return (
            [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
            content.data.into_owned(),
        )
            .into_response();
    }

    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}
