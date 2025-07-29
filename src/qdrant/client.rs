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
    pub vector_size: u32,
}

impl QdrantService {
    pub async fn new(
        qdrant_url: &str,
        collection_name: &str,
        vector_size: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Qdrant::from_url(qdrant_url).build()?;

        let service = Self {
            client: Arc::new(client),
            collection_name: collection_name.to_string(),
            vector_size: vector_size as u32,
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
                    .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine))
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
