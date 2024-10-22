use axum::extract::FromRef;

use crate::{database::Database, search::Tantivy};

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub search: Tantivy,
}

impl AppState {
    pub async fn new(conn_string: String) -> Self {
        let db = Database::new(&conn_string).await;

        let mut search = Tantivy::new().expect("create a new Tantivy instance");
        search.seed(&db).await.unwrap();
        Self { db, search }
    }
}

impl FromRef<AppState> for Database {
    fn from_ref(input: &AppState) -> Self {
        input.db.clone()
    }
}

impl FromRef<AppState> for Tantivy {
    fn from_ref(input: &AppState) -> Self {
        input.search.clone()
    }
}
