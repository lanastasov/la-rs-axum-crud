use axum::{
    extract::{Path, Json, State},
    routing::{get, post, put, delete},
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Book {
    id: Uuid,
    title: String,
    author: String,
}

#[derive(Default)]
struct AppState {
    books: Mutex<Vec<Book>>,
}

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());
    let app = Router::new()
        .route("/books", get(get_books).post(create_book))
        .route("/books/:id", get(get_book).put(update_book).delete(delete_book))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_books(State(state): State<SharedState>) -> Json<Vec<Book>> {
    let books = state.books.lock().unwrap();
    Json(books.clone())
}

async fn get_book(State(state): State<SharedState>, Path(id): Path<Uuid>) -> Result<Json<Book>, StatusCode> {
    let books = state.books.lock().unwrap();
    books.iter().find(|&book| book.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_book(State(state): State<SharedState>, Json(book): Json<Book>) -> StatusCode {
    let mut books = state.books.lock().unwrap();
    books.push(book);
    StatusCode::CREATED
}

async fn update_book(State(state): State<SharedState>, Path(id): Path<Uuid>, Json(updated_book): Json<Book>) -> Result<StatusCode, StatusCode> {
    let mut books = state.books.lock().unwrap();
    match books.iter_mut().find(|book| book.id == id) {
        Some(book) => {
            *book = updated_book;
            Ok(StatusCode::OK)
        },
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_book(State(state): State<SharedState>, Path(id): Path<Uuid>) -> StatusCode {
    let mut books = state.books.lock().unwrap();
    if books.iter().any(|book| book.id == id) {
        books.retain(|book| book.id != id);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
