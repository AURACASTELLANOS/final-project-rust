use axum::{extract::State, response::IntoResponse};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use utoipa::{ToSchema};

#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[utoipa::path(
    get,
    path = "/api/recipes",
    responses(
        (status = 200, description = "List all recipes", body = [Recipe])
    )
)]
pub async fn get_recipes_api(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let recipes = sqlx::query_as::<_, Recipe>("SELECT id, title, description FROM recipes")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    axum::Json(recipes)
}

