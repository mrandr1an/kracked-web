use axum;
/*Tokio*/
//Tracing
use tracing_subscriber;
use tracing::info;
/*Tower*/
use tower::Service;
//STD
use std::net::SocketAddr;
//Custom
mod Application;
use Application::AppRouter::AppRouter;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = AppRouter::new().router;
    let addr = SocketAddr::from(([0,0,0,0], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();  
}

