use axum::{ routing::get, Router, Extension };
use config::AppMode;
use log::info;
use socials_core::db::SocialsDb;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};

pub mod routes;
pub mod config;

#[tokio::main]
async fn main() {
    env_logger::try_init().ok();
    let config = config::parse_args();
    let mode = config.args.mode;
    info!("USING MODE: {:#?}", mode);
    // init db
    let db = match mode {
        AppMode::Dev => SocialsDb::new_test_instance().await.unwrap(),
        AppMode::Prod => SocialsDb::new_instance().await.unwrap(),
    };
    // let db = SocialsDb::new_instance().await.unwrap();

    // init cors
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .expose_headers(Any)
        .allow_origin(Any);

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
