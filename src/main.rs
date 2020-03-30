extern crate pretty_env_logger;
extern crate sqlx;
extern crate anyhow;
#[macro_use] extern crate log;

//use bytes::buf::BufExt;
//use futures_util::{stream, StreamExt};
//use hyper::client::HttpConnector;
pub mod api;
pub mod auth;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use api::response::{root, not_found_response};

extern crate dotenv;

use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

async fn router(
    req: Request<Body>,
    connection: PgPool
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => root().await,
        (&Method::POST, "/auth") => auth::endpoint::post_response(req, &connection).await,
        _ => not_found_response().await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    dotenv().ok();

    let addr = "127.0.0.1:1337".parse().unwrap();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let connection = PgPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(&database_url).await?;
    
    let new_service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        let conn = connection.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                router(req, conn.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    info!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
