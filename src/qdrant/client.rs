use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, ScalarQuantizationBuilder, VectorParamsBuilder};
use qdrant_client::Qdrant;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::types::models::Document;

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
    pub client: Arc<Qdrant>,
    pub collection_name: String,
    pub vector_size: u64,
}

impl QdrantService {
    pub async fn new(
        qdrant_url: &str,
        collection_name: &str,
        vector_size: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let grpc_url = if qdrant_url.contains("6333") {
            qdrant_url.replace("6333", "6334")
        } else {
            format!("http://localhost:6334")
        };

        let client = Qdrant::from_url(&grpc_url).build()?;

        client.health_check().await?;

        let service = Self {
            client: Arc::new(client),
            collection_name: collection_name.to_string(),
            vector_size,
        };

        service.ensure_collection_exists().await?;

        Ok(service)
    }

    pub async fn ensure_collection_exists(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.client.collection_info(&self.collection_name).await {
            Ok(_) => Ok(()), // collection exists
            Err(_) => {
                self.client.create_collection(
                    CreateCollectionBuilder::new(&self.collection_name)
                    .vectors_config(VectorParamsBuilder::new(self.vector_size, Distance::Cosine))
                    .quantization_config(ScalarQuantizationBuilder::default())
                ).await?;
                Ok(())
            }
        }
    }

    pub async fn upsert_document(&self, _document: &Document) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub async fn delete_document(&self, _document: &Document) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub async fn search(&self, _query: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
