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
let keyword_value = keyword.value.clone();

    // Check /var/www/blogs/compsci if contains Keyword
    let compsci_dir = "/var/www/blogs/compsci";
    if check_directory_for_keyword(compsci_dir, &keyword_value) {
        return Json(BlogCard {
            title: String::from("Computer Science Blog"),
            desc: String::from("Found in Computer Science blog"),
        });
    }

    // Else Check /var/www/blogs/misc if contains Keyword
    let misc_dir = "/var/www/blogs/misc";
    if check_directory_for_keyword(misc_dir, &keyword_value) {
        return Json(BlogCard {
            title: String::from("Miscellaneous Blog"),
            desc: String::from("Found in Miscellaneous blog"),
        });
    }

    // Else Check /var/www/blogs/cybersec if contains Keyword
    let cybersec_dir = "/var/www/blogs/cybersec";
    if check_directory_for_keyword(cybersec_dir, &keyword_value) {
        return Json(BlogCard {
            title: String::from("Cybersecurity Blog"),
            desc: String::from("Found in Cybersecurity blog"),
        });
    }

    // Else Check /var/www/blogs/math if contains Keyword
    let math_dir = "/var/www/blogs/math";
    if check_directory_for_keyword(math_dir, &keyword_value) {
        return Json(BlogCard {
            title: String::from("Math Blog"),
            desc: String::from("Found in Math blog"),
        });
    }

    // If none of the directories contain the Keyword, return this BlogCard
    Json(BlogCard {
        title: String::from("No Results"),
        desc: String::from("Please try again"),
    })	
}

fn check_directory_for_keyword(directory_path: &str, keyword: &str) -> bool {
    let keyword_lowercase = keyword.to_lowercase();
    for entry in WalkDir::new(directory_path).into_iter().filter_map(|e| e.ok()) {
        let file_path = entry.path();
        if file_path.is_file() {
            if let Some(file_name) = file_path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.to_lowercase().contains(&keyword_lowercase) {
                        return true;
                    }
                }
            }
        }
    }
    false
}
