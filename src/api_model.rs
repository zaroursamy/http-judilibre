use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JudiResult {
    id: String,
    source: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JudiResponse {
    batch: u8,
    batch_size: u8,
    results: Vec<JudiResult>,
}
