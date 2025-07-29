mod types;
mod qdrant;

use std::sync::Arc;
use types::models::Document;
use qdrant::client::QdrantService;

use axum::{
    extract::State,
    routing::{get, post, delete},
    http::StatusCode,
    Json, Router,
};

// create a _clone_ of the arc pointer - should let us smartly share the qdrant service
// thread-safe across multiple requests
#[derive(Clone)]
struct AppState {
    blog_service: Arc<QdrantService>,
    til_service: Arc<QdrantService>,
}

async fn root() -> &'static str {
    "Semantic Search API"
}

/* Blog */
async fn create_blog_document(
    State(state): State<AppState>,
    Json(document): Json<Document>
) -> Result<(StatusCode, Json<Document>), StatusCode> {
    match state.blog_service.upsert_document(&document).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(document))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_blog_document(
    State(state): State<AppState>,
    Json(document): Json<Document>
) -> Result<StatusCode, StatusCode> {
    match state.blog_service.delete_document(&document).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn search_blogs(
    State(state): State<AppState>,
    Json(query): Json<String>
) -> Result<StatusCode, StatusCode> {
    match state.blog_service.search(query).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// TODO snippets for til, docs for blogs?
/* TIL */
async fn create_til_document(
    State(state): State<AppState>,
    Json(document): Json<Document>
) -> Result<(StatusCode, Json<Document>), StatusCode> {
    match state.til_service.upsert_document(&document).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(document))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_til_document(
    State(state): State<AppState>,
    Json(document): Json<Document>
) -> Result<StatusCode, StatusCode> {
    match state.til_service.delete_document(&document).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn search_tils(
    State(state): State<AppState>,
    Json(query): Json<String>
) -> Result<StatusCode, StatusCode> {
    match state.til_service.search(query).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qdrant_url = "http://localhost:6333";
    let vector_size = 1536;
    let port = 3001;
    let service_url = format!("127.0.0.1:{}", port);

    let blog_service = Arc::new(
        QdrantService::new(qdrant_url, "blogs", vector_size).await?
    );
    let til_service = Arc::new(
        QdrantService::new(qdrant_url, "til", vector_size).await?
    );

    let state = AppState {
        blog_service,
        til_service,
    };

    // use the shared state here
    let app = Router::new()
        .route("/", get(root))
        .route("/blogs", post(create_blog_document))
        .route("/blogs", delete(delete_blog_document))
        .route("/blogs/search", post(search_blogs))
        .route("/til", post(create_til_document))
        .route("/til", delete(delete_til_document))
        .route("/til/search", post(search_tils))
        .with_state(state);

    println!("🚀 Server starting on http://{}", service_url);
    let listener = tokio::net::TcpListener::bind(service_url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
