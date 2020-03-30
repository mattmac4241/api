use hyper::{header, Body, Request, Response, StatusCode};
use bytes::buf::ext::BufExt;
use super::resource::{UserSignup};
use super::db::{create};
//use bytes::Buf;
use bytes::buf::Buf;

use crate::api::response::{bad_request};

use sqlx::postgres::PgPool;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub async fn post_response<'a>(req: Request<Body>, connection: &'a PgPool) -> Result<Response<Body>> {
    let body = hyper::body::aggregate(req).await?;

    // Parse body for user signup
    let parsed = match parse_body(body) {
        Ok(user) => user,
        Err(_) => return Ok(bad_request().unwrap()),
    };

    println!("{}", parsed);
    // Create User
    let id = match create(connection, parsed).await {
        Ok(user_id) => user_id,
        Err(_) => return Ok(bad_request().unwrap()),
    };
    println!("User created with id: {}", id);

    let json: &str = "test";
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(json.into())?)
}

fn parse_body(whole_body: impl Buf) -> Result<UserSignup> {
    let data: UserSignup = serde_json::from_reader(whole_body.reader())?;
    Ok(data)
}
