//Axum
use axum::
{
    response::Html,
    extract::Json,
};
//Serde
use serde::{Deserialize, Serialize};
//Custom Error
use super::error::app_error::AppError;
//Helper Funcs
use super::helper_funcs::retreive_page_untouched;

#[derive(Deserialize, Serialize)]
pub struct Keyword
{
 value: String
}

pub async fn get_blog_search() -> Result<Html<String>, AppError>
{
    retreive_page_untouched("blog-search").await
}

pub async fn post_blog_search(Json(keyword): Json<Keyword>) -> Result<Json<Keyword>, AppError>
{
    Ok(Json(keyword))
}
