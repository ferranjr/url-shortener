use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenedUrl {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nano_id: String,
    pub url: String
}

impl ShortenedUrl {
    pub fn new(
        nano_id: &str,
        url: &str,
    ) -> Self {
        ShortenedUrl {
            id: None,
            nano_id: nano_id.to_string(),
            url: url.to_string(),
        }
    }
}