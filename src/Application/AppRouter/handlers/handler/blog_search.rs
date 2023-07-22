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

pub async fn post_blog_search(Json(keyword): Json<Keyword>) -> Result<Json<Keyword>, AppError>
{
    Ok(Json(keyword))
}

pub async fn read_directory(keyword: Keyword) -> Result<String, AppError>
{
    let search_dir = "/var/www/blogs/";
      // Recursively iterate over the directories and files inside the search directory
    for entry in WalkDir::new(search_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        // Check if the entry is a directory
        if path.is_dir() {
            let dir_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(||{AppError::BadRequest});

            // If the directory name matches the keyword value, return the last two entries of that category
	    if let Ok(dir_name) = dir_name {
		if dir_name == keyword.value
		{
		    let blog_path = search_dir.to_owned() + dir_name;
		    return get_from_category(&blog_path);
		}
		else
		{
		    todo!()
		}
            }
        }
    }
    Err(AppError::BadRequest)
}

fn get_from_category(search_dir: &str) -> Result<String, AppError>
{ 
      // Recursively iterate over the directories and files inside the search directory
    for entry in WalkDir::new(search_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        // Check if the entry is a directory
        if path.is_dir()
	{
	    return Err(AppError::BadRequest)
        }
	else
	{
	    let blog_card = BlogCard
	    {
		title: String::from("My blog"),
		desc: String::from("yeah"),
	    };
	    return Ok(serde_json::to_string(&blog_card).unwrap())
	}
    }
    Err(AppError::BadRequest)
}
