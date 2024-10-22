use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::{
    database::{Database, Note},
    search::{FetchParams, Tantivy},
    state::AppState,
};

pub async fn fetch_notes(
    State(db): State<Database>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match db.fetch_notes().await {
        Ok(notes) => Ok(Json(notes)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn fetch_note_by_id(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match db.fetch_note_by_id(id).await {
        Ok(notes) => Ok(Json(notes)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct NewNote {
    body: String,
}

pub async fn create_note(
    State(mut state): State<AppState>,
    Json(json): Json<NewNote>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let note = match state.db.create_note(json.body).await {
        Ok(note) => note,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    if let Err(e) = state.search.create_doc(note) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    };

    Ok(StatusCode::CREATED)
}

#[derive(Clone, Deserialize)]
pub struct QueryString {
    query: String,
}

pub async fn search(
    State(search): State<Tantivy>,
    Json(json): Json<QueryString>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let params = FetchParams::builder().query(json.query).build();
    match search.fetch_docs(params) {
        Ok(note) => Ok(Json(note)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn health_check() -> &'static str {
    "It works!"
}
