//Std
use std::fs;
use axum::
{
    Router,
    routing::{get, post},
    response::{Html},
    http::StatusCode
};
//tracing
use tracing::{debug, error, info};
//Custom Error
use super::error::app_error::AppError;
use super::error::home_error::HomeError;
/* Returns whole html document */
pub fn home() -> Router
{
    Router::new()
	.route("/", get(home_html))
}

async fn home_html() -> Result<Html<String>, AppError>
{
    info!("Requesting html of homepage");
    if let Ok(html_contents) = fs::read_to_string("frontend/pages/home/index.html")
    {
	if let Ok(mut css_contents) = fs::read_to_string("frontend/pages/home/style/home.css")
	{

	    info!("Adding css of homepage");
	    let css_link = "<link rel=\"stylesheet\" href=\"style/home.css\">";
            css_contents = "<style>".to_owned() + css_contents.as_str() + "</style>";
	    let html_with_css = html_contents
		.replace(css_link, css_contents.as_str());
	    if let Ok(js_contents) = fs::read_to_string("frontend/pages/home/script/main.js")
	    {

		info!("Adding js of homepage");
		let js_link = "<script src=\"script/main.js\"></script>";
		let html_with_css_and_js = html_with_css
		    .replace(js_link, js_contents.as_str());
		Ok(Html(html_with_css_and_js))
	    }
	    //Cant find js file
	    else {
		error!("Could not add js of homepage");
	       Err(AppError::HomeError(HomeError::CouldntLoadJS))
	    }
	}
	//Cant find css file
	else
	{
	    error!("Could not add css of homepage");
	       Err(AppError::HomeError(HomeError::CouldntLoadCSS))
	}
    }
    //Cant find html file
    else
    {
	error!("Could not add html of homepage");
	Err(AppError::HomeError(HomeError::CouldntLoadHTML))
    }
}


