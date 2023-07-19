//Axum
use axum::routing::{get, post,Router};
use axum::{error_handling::HandleErrorLayer, BoxError};
//Tower
use tower_http::services::ServeDir;
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer};
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
//Custom
mod handlers;
use handlers::handler::state;

pub struct AppRouter
{
  pub router: Router, 
}

impl AppRouter
{
    pub fn new() -> Self
    {
	//governor config
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
		.fallback(handlers::handler::fallback::fallback)
		.layer(ServiceBuilder::new()
		// High level logging of requests and responses
		.layer(TraceLayer::new_for_http())
		// this middleware goes above `GovernorLayer` because it will receive
		// errors returned by `GovernorLayer`
		.layer(HandleErrorLayer::new(
		    handlers::handler::ratelimiter::ratelimiter
		))
               .layer(GovernorLayer {
                   // We can leak this because it is created once and then
                   config: Box::leak(governor_conf),
               }),
		)
	}
    }
}
