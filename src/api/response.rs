use hyper::{header, Body, Response, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static BAD_REQUEST_ERROR: &[u8] = b"Bad Request Error";
static NOT_FOUND: &[u8] = b"Not Found";

pub async fn root() -> Result<Response<Body>> {
    let data = format!("Hello and welcome!");
    let res = match serde_json::to_string(&data) {
        Ok(json) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))?,
        Err(_) => internal_server_error().unwrap(),
    };

    Ok(res)
}

pub async fn not_found_response() -> Result<Response<Body>> {
    Ok(Response::builder()
       .status(StatusCode::NOT_FOUND)
       .header(header::CONTENT_TYPE, "application/json")
       .body(NOT_FOUND.into())
       .unwrap())
}

pub fn bad_request() -> Result<Response<Body>> {
    Ok(Response::builder()
       .status(StatusCode::BAD_REQUEST)
       .header(header::CONTENT_TYPE, "application/json")
       .body(BAD_REQUEST_ERROR.into())
       .unwrap())
}

pub fn internal_server_error() -> Result<Response<Body>> {
    Ok(Response::builder()
       .status(StatusCode::INTERNAL_SERVER_ERROR)
       .header(header::CONTENT_TYPE, "application/json")
       .body(INTERNAL_SERVER_ERROR.into())
       .unwrap())
}
