use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub filename: String,
    pub content: String,
}
