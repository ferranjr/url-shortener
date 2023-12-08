use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenUrlRequest {
    pub url: String,
}