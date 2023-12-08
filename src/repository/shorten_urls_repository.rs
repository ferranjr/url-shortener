use async_trait::async_trait;
use bson::extjson::de::Error;
use bson::oid::ObjectId;
use hyper::Uri;
use crate::models::mongo_docs::ShortenedUrl;

#[async_trait]
pub trait ShortenUrlsRepository {
    /// Creates a ShortenedUrl entry, it might fail due to nano_id or url already existing in db
    async fn create_shortened_url(&self, url: Uri) -> Result<ShortenedUrl, Error>;

    /// Given an ObjectId returns the ShortenedUrl entity if this one exists
    async fn get_by_oid(&self, id: ObjectId) -> Result<Option<ShortenedUrl>, Error>;

    /// Given an short code returns the ShortenedUrl entity if this one exists
    async fn get_by_nanoid(&self, nano_id: &str) -> Result<Option<ShortenedUrl>, Error>;

    /// Given a url returns the ShortenedUrl entity if this one exists
    async fn get_by_url(&self, url: Uri) -> Result<Option<ShortenedUrl>, Error>;
}