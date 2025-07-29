use qdrant_client::{
    client::QdrantClient,
    qdrant::{
        CreateCollection, Distance, PointStruct,
        UpsertPoints, vectors::VectorOptions,
        VectorParams,
    },
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query_embedding: Vec<f32>,
    pub limit: Option<u64>,
    pub filters: Option<HashMap<String, String>>,
    pub score_threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub score: f32,
}

pub struct QdrantService {
    client: Arc<QdrantClient>,
    collection_name: String,
    vector_size: u32,
}

impl QdrantService {
    pub async fn new(
        qdrant_url: &str,
        collection_name: &str,
        vector_size: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = QdrantClient::from_url(qdrant_url);

        let service = Self {
            client: Arc::new(client),
            collection_name,
            vector_size,
        };

        service.ensure_collection_exists().await?;

        Ok(service)
    }

    async fn ensure_collection_exists(&self) -> Result<(), Box<dyn std::error::Error>> {

        match self.client.collection_info(&self.collection_name).await {
            Ok(_) => Ok(()), // collection exists
            Err(_) => {
                self.client.create_collection(
                    collection_name: self.collection_name,
                    vectors_config: Some(VectorParams {
                        size: self.vector_size,
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    }.into()),
                    ..Default::default()
            }).await?;
            Ok(())
        }
    }
}
