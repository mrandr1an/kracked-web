use axum::{
    routing::{get, post},
    Router,
};

mod route;

#[tokio::main]
async fn main() {
    let app_router = KrackedRouter::new().routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app_router).await.unwrap();
}

struct KrackedRouter {
    admin: AdminRouter,
    // user: Router
}

struct AdminRouter {
    blog_router: Router,
}

impl KrackedRouter {
    fn new() -> Self {
        Self {
            admin: AdminRouter::new(),
        }
    }

    fn routes(self) -> Router {
        self.admin.routes()
    }
}

impl AdminRouter {
    fn new() -> Self {
        let blog_routes =
            Router::new().route("/admin/publish", post(route::publish::publish_blog_handler));
        Self {
            blog_router: blog_routes,
        }
    }

    fn routes(self) -> Router {
        self.blog_router
    }
}
