//Axum
use axum::
{
    response::Html,
    BoxError
};
use tokio::sync::Mutex;
//tokio
use tokio::time::{sleep, Duration};
//STD
use std::sync::Arc;
//Custom Error
use super::error::app_error::AppError;
use super::error::page_error::PageError;
//Helper Functions
use super::helper_funcs;




pub async fn ratelimiter(b: BoxError) -> Result<Html<String>, AppError>
{
    helper_funcs::retreive_page_untouched("ratelimiter").await
}
