use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, FromRow, Clone)]
pub struct Recipe {
    pub id: i64, // SQLite usa INTEGER AUTOINCREMENT que se mapea como i64
    pub title: String,
    pub ingredients: String, // ingredientes como texto separado por comas
    pub instructions: String,
}

