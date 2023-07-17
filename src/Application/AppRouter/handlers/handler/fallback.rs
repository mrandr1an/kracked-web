//tracing
use tracing::{debug, error, info};
//Custom
use super::error::app_error::AppError;
//Axum
use axum::
{
    handler::Handler,
    response::IntoResponse,
    http::Uri,
};


pub async fn fallback(uri: Uri) -> impl IntoResponse
{
    error!("404 Not Found on uri {}", uri);
    AppError::NotFound.into_response()
}
