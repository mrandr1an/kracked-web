//Axum
use axum::
{
    response::Html,
};

//Custom Error
use super::error::app_error::AppError;

pub async fn get_blog_search() -> Result<Html<String>, AppError>
{
    todo!()
}

pub async fn post_blog_search() -> Result<Html<String>, AppError>
{
    todo!()
}
