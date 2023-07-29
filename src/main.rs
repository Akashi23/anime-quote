mod db;
mod models;
mod routes;

use axum::Router;
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use rand::{rngs::OsRng, RngCore};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use models::quote::Quote;
use models::user::User;
use routes::auth::auth_router;

#[tokio::main]
async fn main() {
    let store = MemoryStore::new();

    let mut secret = [0u8; 128];
    let mut rng = OsRng;
    rng.try_fill_bytes(&mut secret).unwrap();
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    User::init().await.unwrap();
    Quote::init().await.unwrap();

    let app = Router::new().nest("/auth", auth_router()).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    )
    .layer(session_layer);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
