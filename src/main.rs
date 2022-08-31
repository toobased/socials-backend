use axum::{
    extract::{Query, Path},
    routing::get,
    // http::StatusCode,
    response::IntoResponse,
    Json, Router, Extension
};
use socials_core::{db::{SocialsDb, DummyQuery}, tasks::{BotTaskCreate, BotTaskQuery, BotTask}};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // init cors
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .expose_headers(Any)
        .allow_origin(Any);

    // init db
    let db = SocialsDb::new_instance().await.unwrap();

    let app: Router = Router::new()
        .route("/", get(test_route))
        .route("/bots_tasks/",
            get(get_tasks)
            .post(create_task)
        )
        .route("/bots_tasks/:task_id",
            get(get_task)
            .patch(update_task)
            .delete(remove_task)
        )
        .route("/task_types/",
            get(get_task_types)
        )
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(Extension(db))
                .into_inner()
        );

    let addr = SocketAddr::from(([0.0.0.0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn test_route() -> &'static str {
    "Hello there!"
}

async fn get_tasks(
    db: Extension<SocialsDb>,
    query: Option<Query<BotTaskQuery>>
) -> impl IntoResponse {

    let Query(query) = query.unwrap_or_default();
    let result = SocialsDb::find(&query, &db.bots_tasks()).await.unwrap();
    Json(result)
}

async fn get_task(
    db: Extension<SocialsDb>,
    Path(raw_id): Path<uuid::Uuid>
) -> impl IntoResponse {
    let id = bson::Uuid::parse_str(raw_id.to_string()).unwrap();
    let query = BotTaskQuery {
        id: Some(id),
        ..Default::default()
    };
    let result = SocialsDb::find_one(&query, &db.bots_tasks())
        .await.unwrap();
    Json(result)
}

async fn update_task(
    Path(raw_id): Path<uuid::Uuid>,
    db: Extension<SocialsDb>,
    Json(mut updated_task): Json<BotTask>
) -> impl IntoResponse {
    let id = bson::Uuid::parse_str(raw_id.to_string()).unwrap();
    let result = SocialsDb::update_by_id(id, &mut updated_task, &db.bots_tasks())
        .await.unwrap();
    Json(result)
}

async fn create_task(
    db: Extension<SocialsDb>,
    Json(new_task): Json<BotTaskCreate>
) -> impl IntoResponse {
    let task = BotTask::create_from(&db, new_task).await;
    let result = SocialsDb::insert_one(task, db.bots_tasks())
        .await.unwrap();
    Json(result)
}

async fn remove_task(
    db: Extension<SocialsDb>,
    Path(raw_id): Path<uuid::Uuid>
) -> impl IntoResponse {
    let id = bson::Uuid::parse_str(raw_id.to_string()).unwrap();
    let query = BotTaskQuery {
        id: Some(id),
        ..Default::default()
    };
    let result = SocialsDb::delete_many(&query, &db.bots_tasks())
        .await.unwrap();
    Json(result)
}

async fn get_task_types(
    db: Extension<SocialsDb>,
) -> impl IntoResponse {
    let result = SocialsDb::find(&DummyQuery::default(), &db.task_types())
        .await.unwrap();
    Json(result)
}
