//Axum
use axum::
{
    Router,
    routing::{get, post},
    response::{Html},
};
//tracing
use tracing::{info};
//Custom Error
use super::error::app_error::AppError;
//Helper Functions
use super::helper_funcs;
use crate::Application::AppRouter::state::AppState::State;

/* Returns whole html document */
pub fn home() -> Router
{
    info!("Entering home");
    Router::new()
	.route("/", get(home_html))
	.with_state(State::new())
}

async fn home_html() -> Result<Html<String>, AppError>
{
    helper_funcs::retreive_page_untouched("home").await
}


