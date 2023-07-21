//Axum
use axum:: {
    response::{Html},
    extract::Path,
};
//tracing
//Custom Error
use super::error::app_error::AppError;
//Helper Functions
use super::helper_funcs;

pub async fn blog(Path(blog_category): Path<String>, Path(blog_title): Path<String>) -> Result<Html<String>, AppError>
{
    helper_funcs::retrieve_blog(blog_category.as_str(), blog_title.as_str()).await
}

