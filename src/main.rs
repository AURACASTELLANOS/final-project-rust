mod api;

use askama::Template;
use axum::{extract::State, response::{Html, IntoResponse}, routing::get, Router};
use rand::Rng;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Template)]
#[template(path = "recipe.html")]
struct RecipeTemplate {
    title: String,
    ingredients: Vec<String>,
    instructions: String,
}

// Swagger config
#[derive(OpenApi)]
#[openapi(
    paths(api::get_recipes_api),
    components(schemas(api::Recipe)),
    tags(
        (name = "Recipes", description = "Recetas disponibles en la API")
    )
)]
struct ApiDoc;

async fn show_random_recipe(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let recipes = sqlx::query_as::<_, api::Recipe>("SELECT id, title, description FROM recipes")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    if recipes.is_empty() {
        return Html("No hay recetas disponibles.".to_string());
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..recipes.len());
    let recipe = &recipes[index];

    let template = RecipeTemplate {
        title: recipe.title.clone(),
        ingredients: vec![
            "1 huevo".into(),
            "1 taza de harina".into(),
            "1 pizca de sal".into(),
        ],
        instructions: recipe.description.clone(),
    };

    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite:recipes.db").await?;

    let app = Router::new()
        .route("/", get(show_random_recipe))
        .route("/api/recipes", get(api::get_recipes_api))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

