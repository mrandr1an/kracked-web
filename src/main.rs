mod parser;
mod services;

use axum::{routing::get, routing::post, Router};
use services::{blog::blog, home::home, publish::publish};

#[tokio::main]
async fn main() {
    //Routes of website
    let routes = Router::new()
        .route("/", get(home))
        .route("/blog/:blog_title", get(blog))
        .route("/publish", post(publish));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
