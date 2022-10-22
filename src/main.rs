use axum::{ routing::get, Router, Extension };
use socials_core::db::SocialsDb;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};

pub mod routes;

#[tokio::main]
async fn main() {
    // init cors
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .expose_headers(Any)
        .allow_origin(Any);

    // init db
    // TODO make be configurable as params from cli
    let db = SocialsDb::new_test_instance().await.unwrap();
    // let db = SocialsDb::new_instance().await.unwrap();

    let app: Router = Router::new()
        .route("/", get(test_route))

        // tasks router
        .merge(routes::tasks::tasks_router())
        // social source router
        .merge(routes::social_source::social_source_router())
        // bots router
        .merge(routes::bots::bots_router())

        // Common middleware for all routes
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(Extension(db))
                .into_inner()
        );

    let addr = SocketAddr::from(([0,0,0,0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn test_route() -> &'static str { "Hello there!" }
