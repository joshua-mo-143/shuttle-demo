use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    database::{Database, Note},
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
    State(state): State<Database>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match db.fetch_note_by_id(id).await {
        Ok(notes) => Ok(Json(notes)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_note(State(state): State<AppState>, Json(json): Json<Note>) {}
pub async fn delete_note() {}
