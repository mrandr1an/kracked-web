mod parser;
mod services;
use axum::{routing::get, routing::post, Router};
use services::{blog::blog, home::home, publish::publish};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    //Routes of website
    let routes = Router::new()
        .route("/", get(home))
        .route("/blog/:blog_title", get(blog))
        .route("/publish", post(publish))
        .route_service(
            "/blog/dependencies/css/styler.css",
            ServeFile::new("src/theme/dependencies/css/styler.css"),
        )
        .route_service(
            "/blog/dependencies/css/bootstrap.min.css",
            ServeFile::new("src/theme/dependencies/css/bootstrap.min.css"),
        )
        .route_service(
            "/blog/dependencies/js/bootstrap.min.js",
            ServeFile::new("src/theme/dependencies/js/bootstrap.min.js"),
        )
        .route_service(
            "/blog/dependencies/js/prism.js",
            ServeFile::new("src/theme/dependencies/js/prism.js"),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
