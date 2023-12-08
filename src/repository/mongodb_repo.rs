use std::env;
use async_trait::async_trait;

extern crate dotenv;

use dotenv::dotenv;
use hyper::Uri;

use mongodb::{bson::{doc, extjson::de::Error, oid::ObjectId}, Client, Collection};
use nanoid::nanoid;

use crate::models::mongo_docs::ShortenedUrl;
use crate::repository::shorten_urls_repository::ShortenUrlsRepository;

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
}

#[async_trait]
impl ShortenUrlsRepository for MongoRepo {
    async fn create_shortened_url(&self, url: Uri) -> Result<ShortenedUrl, Error> {
        let nano_id = nanoid!(8, &nanoid::alphabet::SAFE);
        let new_doc = ShortenedUrl {
            id: None,
            nano_id,
            url: url.to_string(),
        };
        let shortened_url_id = self
            .col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating shortened url entry")
            .inserted_id
            .as_object_id()
            .unwrap();

        let result = self.get_by_oid(shortened_url_id).await?.unwrap();
        Ok(result)
    }

    async fn get_by_oid(&self, id: ObjectId) -> Result<Option<ShortenedUrl>, Error> {    //     let filter = doc! {"_id": id};
        let filter = doc! {"_id": id};
        let shortened_url = self
            .col
            .find_one(filter, None)
            .await
            .expect("Error getting shortened url's detail");

        Ok(shortened_url)
    }

    async fn get_by_nanoid(&self, nano_id: &str) -> Result<Option<ShortenedUrl>, Error> {
        let filter = doc! {"nano_id": nano_id};
        let shortened_url = self
            .col
            .find_one(filter, None)
            .await
            .expect("Error getting shortened url's detail");

        Ok(shortened_url)
    }

    async fn get_by_url(&self, url: Uri) -> Result<Option<ShortenedUrl>, Error> {
        let filter = doc! { "url": url.to_string() };
        let shortened_url = self
            .col
            .find_one(filter, None)
            .await
            .expect("Error getting shortened url's detail");
        Ok(shortened_url)
    }
}