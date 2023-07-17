//Std
use std::fs;
use axum::response::Html;

//tracing
use tracing::{debug, error, info};
//Custom Error
use super::error::page_error::PageError;
use super::error::app_error::AppError;

pub async fn retreive_page_untouched
    (name: &str) -> Result<Html<String>, AppError>
{
    info!("Retreiving page {}", name);
    let path_html = format!("frontend/pages/{name}/{name}.html");

    let path_css = format!("frontend/pages/{name}/style/{name}.css");
    let path_js = format!("frontend/pages/{name}/script/main.js");
    if let Ok(html_contents) = fs::read_to_string(path_html)
    {
	if let Ok(mut css_contents) = fs::read_to_string(path_css)
	{

	    info!("Processing: Adding css of {}",name);
	    let css_link = "<link rel=\"stylesheet\" href=\"style/home.css\">";
            css_contents = "<style>".to_owned() + css_contents.as_str() + "</style>";
	    let html_with_css = html_contents
		.replace(css_link, css_contents.as_str());
	    if let Ok(js_contents) = fs::read_to_string(path_js)
	    {

                info!("Processing: Adding js of {}",name);
		let js_link = "<script src=\"script/main.js\"></script>";
		let html_with_css_and_js = html_with_css
		    .replace(js_link, js_contents.as_str());
		Ok(Html(html_with_css_and_js))
	    }
	    //Cant find js file
	    else {
		error!("Could not add js of {}", name);
	       Err(AppError::PageError(PageError::CouldntLoadJS))
	    }
	}
	//Cant find css file
	else
	{
		error!("Could not add css of {}", name);
	       Err(AppError::PageError(PageError::CouldntLoadCSS))
	}
    }
    //Cant find html file
    else
    {
	error!("Could not add HTML of {}", name);
	Err(AppError::PageError(PageError::CouldntLoadHTML))
    }
}
