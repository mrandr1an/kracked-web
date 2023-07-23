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
//Walkdir
use walkdir::WalkDir;

#[derive(Deserialize, Serialize)]
pub struct Keyword
{
 value: String
}

#[derive(Deserialize, Serialize)]
pub struct BlogCard
{
    title: String,
    desc: String,
}

pub async fn get_blog_search() -> Result<Html<String>, AppError>
{
    retreive_page_untouched("blog-search").await
}

pub async fn post_blog_search(Json(keyword): Json<Keyword>) -> Json<BlogCard>
{
    Json(BlogCard { title: String::from(keyword.value), desc: String::from("some") })
    //Check /var/www/blogs/compsci if contains Keyword

    //Check /var/www/blogs/misc if contains Keyword

    //Check /var/www/blogs/cybersec if contains Keyword

   //Check /var/www/blogs/math if contains Keyword


	
}

