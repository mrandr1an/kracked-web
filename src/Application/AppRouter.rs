//Axum
use axum::routing::{get, post,Router};
use axum::{error_handling::HandleErrorLayer, BoxError};
//Tower
use tower_http::services::ServeDir;
use tower::ServiceBuilder;
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
//Custom
mod handlers;

pub struct AppRouter
{
  pub router: Router, 
}

impl AppRouter
{
    pub fn new() -> Self
    {
	 let governor_conf = Box::new(
       GovernorConfigBuilder::default()
           .per_second(2)
           .burst_size(5)
           .finish()
           .unwrap(),
	 );
	AppRouter{
	    router:Router::new()
		.nest("/home",
		      handlers::handler::home::home())
		.layer(ServiceBuilder::new()
               // this middleware goes above `GovernorLayer` because it will receive
               // errors returned by `GovernorLayer`
               .layer(HandleErrorLayer::new(|e: BoxError| async move {
                   display_error(e)
               }))
               .layer(GovernorLayer {
                   // We can leak this because it is created once and then
                   config: Box::leak(governor_conf),
               }),
		)
	}
    }
}
