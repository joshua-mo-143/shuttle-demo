use axum::{
    routing::{get, post},
    Router,
};
use state::AppState;
use tokio::net::TcpListener;

pub mod database;
pub mod endpoints;
pub mod search;
pub mod state;

#[tokio::main]
async fn main() {
    let conn_string = std::env::var("DATABASE_URL").expect("DATABASE_URL env var to exist");
    let state = AppState::new(conn_string).await;

    let router = Router::new()
        .route("/healthz", get(endpoints::health_check))
        .route("/search", post(endpoints::search))
        .route(
            "/notes",
            get(endpoints::fetch_notes).post(endpoints::create_note),
        )
        .route("/note/:id", get(endpoints::fetch_note_by_id))
        .with_state(state);

    let tcp = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("to create a TcpListener");

    axum::serve(tcp, router)
        .await
        .expect("to serve an Axum router");
}
