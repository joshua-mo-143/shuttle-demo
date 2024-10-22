use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Clone)]
pub struct Database {
    db: PgPool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Note {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub body: String,
}

impl Database {
    pub async fn new(conn_string: &str) -> Self {
        let db = PgPool::connect(conn_string)
            .await
            .expect("to connect to the database");

        Self { db }
    }

    pub async fn seed(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!().run(&self.db).await?;

        Ok(())
    }

    pub async fn fetch_notes(&self) -> Result<Vec<Note>, sqlx::Error> {
        let query: Vec<Note> = sqlx::query_as("SELECT * FROM notes")
            .fetch_all(&self.db)
            .await?;

        Ok(query)
    }

    pub async fn fetch_note_by_id(&self, id: i64) -> Result<Note, sqlx::Error> {
        let query: Note = sqlx::query_as("SELECT * FROM notes where id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        Ok(query)
    }

    pub async fn create_note(&self, body: String) -> Result<Note, sqlx::Error> {
        let query: Note = sqlx::query_as("INSERT INTO notes (body) VALUES ($1) returning *")
            .bind(body)
            .fetch_one(&self.db)
            .await?;

        Ok(query)
    }
}
