use std::io;
use hyper::body::Buf;

mod models;
mod repository;

use std::net::SocketAddr;
use std::time::Duration;
use tokio::pin;
use http_body_util::{BodyExt, combinators::BoxBody, Empty, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::{Method, Request, Response, StatusCode};
use hyper::header::LOCATION;
use hyper::http::HeaderValue;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;
use nanoid::nanoid;
use crate::models::shorten_url_request::ShortenUrlRequest;
use crate::models::mongo_docs::ShortenedUrl;
use crate::repository::mongodb_repo::MongoRepo;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Instantiate mongo repo
    let db = MongoRepo::init().await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;

    // Use a 5 second timeout for incoming connections to the server.
    // If a request is in progress when the 5 second timeout elapses,
    // use a 2 second timeout for processing the final request and graceful shutdown.
    let connection_timeouts = vec![Duration::from_secs(5), Duration::from_secs(2)];

    // Todo: use one of the logger libraries
    println!("Listening on http://{}", addr);

    loop {
        // When an incoming TCP connection is received grab a TCP stream for
        // client<->server communication.
        let (tcp, remote_address) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(tcp);

        // Clone the connection_timeouts so they can be passed to the new task.
        let connection_timeouts_clone = connection_timeouts.clone();

        // Print the remote address connecting to our server.
        println!("accepted connection from {:?}", remote_address);

        let db = db.clone();
        tokio::task::spawn(async move {
            // Pin the connection object so we can use tokio::select! below.
            let conn = http1::Builder::new()
                .serve_connection(io, service_fn(|r| handle(&db, r)));
            pin!(conn);

            // Iterate the timeouts.  Use tokio::select! to wait on the
            // result of polling the connection itself,
            // and also on tokio::time::sleep for the current timeout duration.
            for (iter, sleep_duration) in connection_timeouts_clone.iter().enumerate() {
                tokio::select! {
                    res = conn.as_mut() => {
                        // Polling the connection returned a result.
                        // In this case print either the successful or error result for the connection
                        // and break out of the loop.
                        match res {
                            Ok(()) => println!("after polling conn, no error"),
                            Err(err) =>  println!("error serving connection: {:?}", err),
                        };
                        break;
                    }
                    _ = tokio::time::sleep(*sleep_duration) => {
                        // tokio::time::sleep returned a result.
                        // Call graceful_shutdown on the connection and continue the loop.
                        println!("iter = {} got timeout_interval, calling conn.graceful_shutdown", iter);
                        conn.as_mut().graceful_shutdown();
                    }
                }
            }
        });
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

async fn handle(
    db: &MongoRepo,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let method = req.method();
    let path = req.uri().path().split('/').skip(1).collect::<Vec<_>>();

    match (method, &*path) {
        // Status endpoint
        (&Method::GET, ["private", "status"]) => Ok(Response::new(full("OK"))),
        // Retrieve a specific code
        (&Method::GET, [code]) if !code.is_empty() => {
            let res = db.get_full_url(code).await;

            res.map(|maybe_url| {
                match maybe_url {
                    Some(url) => {
                        let mut empty_response = Response::new(empty());
                        *empty_response.status_mut() = StatusCode::PERMANENT_REDIRECT;
                        empty_response.headers_mut().insert(
                            LOCATION,
                            HeaderValue::from_str(url.url.as_str()).unwrap(),
                        );
                        Ok(empty_response)
                    }
                    None => {
                        let mut not_found = Response::new(empty());
                        *not_found.status_mut() = StatusCode::NOT_FOUND;
                        Ok(not_found)
                    }
                }
            }).unwrap()
        }
        (&Method::POST, ["short"]) => {
            let whole_body = req.collect().await?.to_bytes();
            let shorten_url_req: ShortenUrlRequest = serde_json::from_reader(whole_body.reader())
                .map_err(|e| io::Error::new(
                    io::ErrorKind::Other,
                    e,
                )
                ).unwrap();

            // TODO: logic to create the shortened should be in a service that uses the repo,
            // so it can handle the writeException to recreate and retry
            let short_id = nanoid!(8, &nanoid::alphabet::SAFE);
            let shortened_url_id = db.create_shortened_url(ShortenedUrl::new(&short_id, &shorten_url_req.url)).await.unwrap();
            let result = db.get_by_id(shortened_url_id).await.unwrap()
                .map(|shortened_url| {
                    Response::new(full(format!("Your short url http://localhost:8080/{}", shortened_url.nano_id.as_str())))
                }).unwrap();
            Ok(result)
        }
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}