//! Request-id middleware: propagates or generates a correlation id per request.

use axum::extract::Request;
use axum::http::{HeaderName, HeaderValue};
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct RequestId(pub Uuid);

pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| Uuid::parse_str(value).ok())
        .unwrap_or_else(Uuid::new_v4);

    request.extensions_mut().insert(RequestId(request_id));

    let mut response = next.run(request).await;

    if let Ok(value) = HeaderValue::from_str(&request_id.to_string()) {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-request-id"), value);
    }

    response
}
