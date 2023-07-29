use axum::http::Request;

use axum::{
    http::StatusCode, 
    routing::post, Json,
    body::Body,
    Router
};
use axum_sessions::SessionHandle;
use axum_sessions::async_session::serde_json;
use axum_sessions::extractors::WritableSession;

use serde::Deserialize;

use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};

use crate::models;
use models::user::User;

#[derive(Deserialize)]
struct AuthUser {
    username: String,
    password: String,
}

#[axum_macros::debug_handler]
pub async fn register(
    request : Request<Body>,
) -> (StatusCode, Json<User>) {
    let hash_string = "$argon2i$v=19$m=65536,t=1,p=1$c29tZXNhbHQAAAAAAAAAAA$+r0d29hqEB0yasKr55ZgICsQGSkl0v0kgwhd+U3wyRo";
    let password_hash = PasswordHash::new(&hash_string).expect("invalid password hash");
    
    let (parts, body) = request.into_parts();
    
    let payload = hyper::body::to_bytes(body).await.unwrap();
    let payload = serde_json::from_slice::<AuthUser>(&payload).unwrap();
    
    let user = User::new(payload.username, password_hash.to_string()).await;
    let user = match user {
        Ok(user) => user,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(User::default())),
    };
    
    let session_handle = parts.extensions.get::<SessionHandle>().unwrap();
    let mut session = session_handle.write().await;
    session.insert("user_id", user.id).unwrap();
    
    (StatusCode::CREATED, Json(user))
}

#[axum_macros::debug_handler]
pub async fn login(
    request : Request<Body>,
) -> (StatusCode, Json<User>) {
    let (parts, body) = request.into_parts();
    
    let payload = hyper::body::to_bytes(body).await.unwrap();
    let payload = serde_json::from_slice::<AuthUser>(&payload).unwrap();

    let user = User::find_by_username(payload.username).await;
    let user = match user {
        Ok(user) => user,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(User::default())),
    };

    let session_handle = parts.extensions.get::<SessionHandle>().unwrap();
    let mut session = session_handle.write().await;

    let password_hash = PasswordHash::new(&user.password).expect("invalid password hash");
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];
    if password_hash
        .verify_password(algs, payload.password)
        .is_ok()
    {
        session.insert("user_id", user.id).unwrap();
        (StatusCode::OK, Json(user))
    } else {
        (StatusCode::UNAUTHORIZED, Json(user))
    }
}

pub async fn logout(mut session: WritableSession) -> (StatusCode, &'static str) {
    session.destroy();
    (StatusCode::OK, "Logged out")
}

pub fn auth_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
}
