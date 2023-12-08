use std::env;
extern crate dotenv;

use dotenv::dotenv;

use mongodb::{bson::{doc, extjson::de::Error, oid::ObjectId}, Client, Collection};

use crate::models::mongo_docs::ShortenedUrl;

#[derive(Clone)]
pub struct MongoRepo {
    col: Collection<ShortenedUrl>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("shortUrlsDb");
        let col: Collection<ShortenedUrl> = db.collection("shortUrlsDb");
        MongoRepo { col }
    }

    pub async fn create_shortened_url(&self, new_shortened_url: ShortenedUrl) -> Result<ObjectId, Error> {
        let new_doc = ShortenedUrl {
            id: None,
            nano_id: new_shortened_url.nano_id,
            url: new_shortened_url.url,
        };
        let shortened_url_id = self
            .col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating shortened url entry")
            .inserted_id
            .as_object_id()
            .unwrap();

        Ok(shortened_url_id)
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<ShortenedUrl>, Error> {
        let filter = doc! {"_id": id};
        let shortened_url = self
            .col
            .find_one(filter, None)
            .await
            .expect("Error getting shortened url's detail");

        Ok(shortened_url)
    }

    pub async fn get_full_url(&self, nano_id: &str) -> Result<Option<ShortenedUrl>, Error> {
        let filter = doc! {"nano_id": nano_id};
        let shortened_url = self
            .col
            .find_one(filter, None)
            .await
            .expect("Error getting shortened url's detail");

        Ok(shortened_url)
    }

}