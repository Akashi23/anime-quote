use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::{get, post, patch, delete},
    Json, Router,
};
use axum_sessions::{async_session::serde_json, extractors::WritableSession, SessionHandle};
use hyper::http::request;
use serde::{Deserialize, Serialize};


use crate::models::quote::Quote;

#[derive(Serialize, Deserialize)]
pub struct QuoteWithoutId {
    pub quote: String,
    pub anime: String,
    pub character: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuoteWithId {
    pub id: i32,
    pub quote: String,
    pub anime: String,
    pub character: String,
}

impl Default for QuoteWithId {
    fn default() -> Self {
        Self {
            id: 0,
            quote: String::new(),
            anime: String::new(),
            character: String::new(),
        }
    }
}

impl From<Quote> for QuoteWithId {
    fn from(quote: Quote) -> Self {
        Self {
            id: quote.id,
            quote: quote.quote,
            anime: quote.anime,
            character: quote.character,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

pub async fn create_quote(request: Request<Body>) -> (StatusCode, Json<QuoteWithId>) {
    let (parts, body) = request.into_parts();

    let session_handle = parts.extensions.get::<SessionHandle>().unwrap();
    let session = session_handle.read().await;
    let user_id = session.get::<i32>("user_id").unwrap();

    let payload = hyper::body::to_bytes(body).await.unwrap();
    let payload = serde_json::from_slice::<QuoteWithoutId>(&payload).unwrap();

    let quote = Quote::new(user_id, payload.quote, payload.anime, payload.character).await;
    let quote = match quote {
        Ok(quote) => quote,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(Default::default())),
    };

    (StatusCode::CREATED, Json(From::from(quote)))
}

pub async fn get_quotes(request: Request<Body>) -> (StatusCode, Json<Vec<QuoteWithId>>) {
    let (parts, _) = request.into_parts();

    let session_handle = parts.extensions.get::<SessionHandle>().unwrap();
    let session = session_handle.read().await;
    let user_id = session.get::<i32>("user_id").unwrap();

    let quotes = Quote::find_by_user_id(user_id).await;
    let quotes = match quotes {
        Ok(quotes) => quotes,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(Default::default())),
    };

    let quotes = quotes.into_iter().map(|quote| quote.into()).collect();

    (StatusCode::OK, Json(quotes))
}

pub async fn read_quote(request: Request<Body>) -> (StatusCode, Json<QuoteWithId>) {
    let (parts, _) = request.into_parts();

    let session_handle = parts.extensions.get::<SessionHandle>().unwrap();
    let session = session_handle.read().await;
    let user_id = session.get::<i32>("user_id").unwrap();

    let id = parts
        .extensions
        .get::<axum::extract::Extension<i32>>()
        .unwrap()
        .clone();

    let quote = Quote::find_by_id(id).await;
    let quote = match quote {
        Ok(quote) => quote,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(Default::default())),
    };

    if quote.user_id != user_id {
        return (StatusCode::UNAUTHORIZED, Json(Default::default()));
    }

    (StatusCode::OK, Json(From::from(quote)))
}



pub fn quote_router() -> Router {
    Router::new()
        .route("/quotes", post(create_quote))
        .route("/quotes", get(get_quotes))
        .route("/quotes/:id", patch(edit_quote))
        .route("/quotes/:id", get(read_quote))
        .route("/quotes/:id", delete(delete_quote))
}
