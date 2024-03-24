use anyhow::Result;
use axum::{routing::get, Extension, Router};
use dotenv::dotenv;
use std::env;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use common::db;
use common::db::create_db_connection;

pub mod handlers;

use std::sync::Arc;

pub struct AppState {
    pub db: db::SurrealDb,
}

impl AppState {
    fn default(db: db::SurrealDb) -> Self {
        AppState { db }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() -> Result<()> {
    // Load env vars from .env file
    dotenv().ok();

    let db = create_db_connection().await.unwrap();

    let shared_state: SharedState = Arc::new(RwLock::new(AppState::default(db)));

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/block/:block_id", get(handlers::blocks::get_block))
        .route(
            "/block_number/:block_id",
            get(handlers::blocks::get_block_number),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(shared_state)),
        );

    let port = env::var("PORT").unwrap_or(3000.to_string());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Axum is rippin!!!"
}
