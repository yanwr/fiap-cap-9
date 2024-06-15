use axum::body::Body;
use axum::http::{Method, Request, StatusCode, Uri};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use http_body_util::BodyExt;
use tracing::log::{log_enabled, Level};
use tracing::{error, info};

pub async fn log_response(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let res = next.run(req).await;

    let status = res.status();
    let (parts, body) = res.into_parts();

    let bytes = buffer_and_print(method, uri, status, body).await;

    Ok(Response::from_parts(parts, Body::from(bytes)))
}

async fn buffer_and_print(method: Method, uri: Uri, status: StatusCode, body: Body) -> Bytes {
    let body_bytes = body
        .collect()
        .await
        .expect("Failed to read body bytes")
        .to_bytes();

    let body_string = if log_enabled!(Level::Debug) {
        std::str::from_utf8(&body_bytes)
            .unwrap_or("\"redacted\"")
            .to_string()
    } else {
        "\"redacted\"".to_string()
    };

    if status.is_server_error() {
        error!(
            "{method} {uri} -> {} :: response :: {body_string}",
            status.as_u16(),
        );
    } else {
        info!(
            "{method} {uri} -> {} :: response :: {body_string}",
            status.as_u16(),
        );
    }

    body_bytes
}
