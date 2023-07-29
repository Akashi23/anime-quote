mod routes;
mod models;
mod db;

use axum::Router;
use axum_sessions::{
    async_session::MemoryStore,
    SessionLayer,
};
use rand::{rngs::OsRng, RngCore};

use routes::auth::auth_router;
use models::user::User;
use models::quote::Quote;


#[tokio::main]
async fn main() {
    let store = MemoryStore::new();

    let mut secret = [0u8; 128];
    let mut rng = OsRng;
    rng.try_fill_bytes(&mut secret).unwrap();
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);

    User::init().await.unwrap();
    Quote::init().await.unwrap();

    let app = Router::new()
        .nest("/auth", auth_router())
        .layer(session_layer);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}