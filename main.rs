mod types;
use types::models::{Document};

mod qdrant;
use qdrant::client::{
    QdrantService, QdrantClient
};

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
    let app: Router<()> = Router::new()
        .route("/", get(root))
        .route("/document", post(create_document));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
