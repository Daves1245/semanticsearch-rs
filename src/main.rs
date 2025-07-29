mod types;
mod qdrant;

use types::models::Document;
use qdrant::client::QdrantService;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

/* Axum example */
async fn root() -> &'static str {
    "Hello, world!"
}

async fn create_document(Json(payload): Json<Document>) -> (StatusCode, Json<Document>) {
    (StatusCode::CREATED, Json(payload))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let qdrant_url = "";
    let vector_size = 3280;

    let app: Router<()> = Router::new()
        .route("/", get(root))
        .route("/document", post(create_document));

    let blog_client = QdrantService::new(
        qdrant_url,
        "blogs",
        vector_size,
    ).await?;

    let til_client = QdrantService::new(
        qdrant_url,
        "til",
        vector_size
    ).await?;

    blog_client.ensure_collection_exists().await?;
    til_client.ensure_collection_exists().await?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
