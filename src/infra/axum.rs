use crate::infra::errors::AppError;
use crate::infra::observability::Tags;
use axum::extract::FromRequest;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use bytes::{BufMut, BytesMut};
use serde::Serialize;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJsonRequest<T>(pub T);

pub struct AppJsonResponse<T> {
    body: T,
    tags: Tags,
}

impl<T> AppJsonResponse<T> {
    pub fn new(body: T) -> Self {
        AppJsonResponse {
            body,
            tags: Tags::default(),
        }
    }

    pub fn with_tags(mut self, tags: Tags) -> Self {
        self.tags = tags;
        self
    }
}

const JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";
const TEXT_PLAIN_CONTENT_TYPE: &str = "text/plain; charset=utf-8";

impl<T> IntoResponse for AppJsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let mut buf = BytesMut::with_capacity(128).writer();
        match serde_json::to_writer(&mut buf, &self.body) {
            Ok(()) => {
                let bytes = buf.into_inner().freeze();

                let response =
                    ([(header::CONTENT_TYPE, JSON_CONTENT_TYPE)], bytes.clone()).into_response();

                let (mut parts, body) = response.into_parts();
                parts.extensions.insert(self.tags);

                Response::from_parts(parts, body)
            }
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, TEXT_PLAIN_CONTENT_TYPE)],
                err.to_string(),
            )
                .into_response(),
        }
    }
}
