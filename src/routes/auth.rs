use axum::http::Request;

use axum::routing::get;
use axum::{body::Body, http::StatusCode, routing::post, Json, Router};
use axum_sessions::async_session::serde_json;
use axum_sessions::extractors::WritableSession;
use axum_sessions::SessionHandle;

use serde::{Deserialize, Serialize};

use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};

use crate::models;
use models::user::User;

#[derive(Deserialize, Debug)]
pub struct AuthUser {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserWithoutPassword {
    id: i32,
    username: String,
}

impl Default for UserWithoutPassword {
    fn default() -> Self {
        UserWithoutPassword {
            id: -1,
            username: String::from(""),
        }
    }
}

impl From<User> for UserWithoutPassword {
    fn from(user: User) -> Self {
        UserWithoutPassword {
            id: user.id,
            username: user.username,
        }
    }
}

#[axum_macros::debug_handler]
pub async fn register(request: Request<Body>) -> (StatusCode, Json<UserWithoutPassword>) {
    let hash_string = "$argon2i$v=19$m=65536,t=1,p=1$c29tZXNhbHQAAAAAAAAAAA$+r0d29hqEB0yasKr55ZgICsQGSkl0v0kgwhd+U3wyRo";
    let password_hash = PasswordHash::new(&hash_string).expect("invalid password hash");

    let (parts, body) = request.into_parts();

    let payload = hyper::body::to_bytes(body).await.unwrap();
    let payload = serde_json::from_slice::<AuthUser>(&payload).unwrap();

    let user = User::new(payload.username, password_hash.to_string()).await;
    let user = match user {
        Ok(user) => user,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(Default::default())),
    };

    let session_handle = parts.extensions.get::<SessionHandle>().unwrap();
    let mut session = session_handle.write().await;
    session.insert("user_id", user.id).unwrap();

    (StatusCode::CREATED, Json(From::from(user)))
}

#[axum_macros::debug_handler]
pub async fn login(request: Request<Body>) -> (StatusCode, Json<UserWithoutPassword>) {
    let (parts, body) = request.into_parts();

    let payload = hyper::body::to_bytes(body).await.unwrap();
    let payload = serde_json::from_slice::<AuthUser>(&payload).unwrap();
    let user = User::find_by_username(payload.username).await;
    tracing::info!("user: {:?}", user);
    let user = match user {
        Ok(user) => user,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(Default::default())),
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
        (StatusCode::OK, Json(From::from(user)))
    } else {
        (StatusCode::UNAUTHORIZED, Json(From::from(user)))
    }
}

#[derive(Serialize)]
pub struct Message {
    message: String,
}
pub async fn logout(mut session: WritableSession) -> (StatusCode, Json<Message>) {
    session.destroy();
    (
        StatusCode::OK,
        Json(Message {
            message: String::from("Logged out"),
        }),
    )
}

pub fn auth_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
}
