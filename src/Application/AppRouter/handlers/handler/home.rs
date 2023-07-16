//Std
use std::fs;
use axum::
{
    Router,
    routing::{get, post},
    response::{Html},
    http::StatusCode
};

/* Returns whole html document */
pub fn home() -> Router
{
    Router::new()
	.route("/", get(home_html))
}

async fn home_html() -> Result<Html<String>, StatusCode>
{

    if let Ok(html_contents) = fs::read_to_string("frontend/pages/home/index.html")
    {
	if let Ok(mut css_contents) = fs::read_to_string("frontend/pages/home/style/home.css")
	{
	    let css_link = "<link rel=\"stylesheet\" href=\"style/home.css\">";
            css_contents = "<style>".to_owned() + css_contents.as_str() + "</style>";
	    let html_with_css = html_contents
		.replace(css_link, css_contents.as_str());
	    if let Ok(js_contents) = fs::read_to_string("frontend/pages/home/script/main.js")
	    {
		let js_link = "<script src=\"script/main.js\"></script>";
		let html_with_css_and_js = html_with_css
		    .replace(js_link, js_contents.as_str());
		Ok(Html(html_with_css_and_js))
	    }
	    //Cant find js file
	    else {
	       Err(StatusCode::NOT_FOUND)
	    }
	}
	//Cant find css file
	else
	{
	    Err(StatusCode::NOT_FOUND)
	}
    }
    //Cant find html file
    else
    {
	Err(StatusCode::NOT_FOUND)
    }
}


