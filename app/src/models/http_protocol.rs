use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenUrlRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenedUrlResponse {
    pub url: String,
    pub short_url: String,
}