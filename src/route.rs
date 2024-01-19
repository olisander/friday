use askama_axum::IntoResponse;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::template::{HelloTemplate, ModalTemplate};

pub async fn index(State(pool): State<SqlitePool>) -> impl IntoResponse {
    HelloTemplate {}
}

pub async fn modal(State(pool): State<SqlitePool>) -> impl IntoResponse {
    ModalTemplate {}
}
