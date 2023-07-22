//STD
use std::fs;
//Axum
use axum::{
    response::{IntoResponse, Html},
};

//Tracing
use tracing::{debug, error, info};
//Custom
use super::page_error::PageError;

pub enum AppError
{
    PageError(PageError),
    BadRequest,
    NotFound,
}

impl IntoResponse for AppError
{
    fn into_response(self) -> axum::response::Response
    {
    error!("AppError detected: {}", self);
    info!("Requesting html of errorpage");
    if let Ok(html_contents) = fs::read_to_string("frontend/pages/error/error.html")
    {
	if let Ok(mut css_contents) = fs::read_to_string("frontend/pages/error/style/error.css")
	{

	    info!("Adding css of errorpage");
	    let css_link = "<link rel=\"stylesheet\" href=\"style/error.css\">";
            css_contents = "<style>".to_owned() + css_contents.as_str() + "</style>";
	    let html_with_css = html_contents
		.replace(css_link, css_contents.as_str());
	    if let Ok(mut js_contents) = fs::read_to_string("frontend/pages/error/script/main.js")
	    {

		info!("Adding js of errorpage");
		let js_link = "<script src=\"script/main.js\"></script>";
		js_contents = "<script>".to_owned() + js_contents.as_str() + "</script>";
		let html_with_css_and_js = html_with_css
		    .replace(js_link, js_contents.as_str());
		return Html(html_with_css_and_js).into_response();
	    }
	    //Cant find js file
	    else {
		error!("Could not add js of errorpage");
		return Html(html_with_css).into_response();
	    }
	}
	//Cant find css file
	else
	{
	    error!("Could not add css of errorpage");
	    return Html(html_contents).into_response();
	}
    }
    //Cant find html file
    else
    {
	error!("Could not add html of errorpage");
	"Not found".into_response()
    }
    }
}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::PageError(v) => write!(f, "AppError : HomeError : {}", v ),
            AppError::NotFound => write!(f, "AppError : NotFound"),
	    AppError::BadRequest => write!(f,"AppError : BadRequest"),
	    _ => write!(f, "Unknown error")
        }
    }
}
