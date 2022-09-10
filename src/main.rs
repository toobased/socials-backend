use axum::{
    extract::{Query, Path},
    routing::get,
    response::IntoResponse,
    Json, Router, Extension
};
use socials_core::{db::{SocialsDb, DummyQuery }, tasks::{BotTaskCreate, BotTaskQuery, BotTask}, social::source::{SocialSourceQuery, SocialSource, SocialSourceCreate}};
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
    // todo
    // let db = SocialsDb::new_test_instance().await.unwrap();
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
        .route("/social_sources/",
            get(get_social_sources)
            .post(create_social_sources)
        )
        .route("/social_sources/:source_id",
            get(get_social_source)
            .delete(remove_social_sources)
            .patch(update_social_source)
        )
        // Add middleware to all routes
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
    Json(updated_task): Json<BotTask>
) -> impl IntoResponse {
    let id = bson::Uuid::parse_str(raw_id.to_string()).unwrap();
    let result = SocialsDb::update_by_id(id, updated_task.clone(), &db.bots_tasks())
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

async fn get_social_sources(
    db: Extension<SocialsDb>
) -> impl IntoResponse {
    let result = SocialsDb::find(&DummyQuery::default(), &db.social_sources())
        .await.unwrap();
    Json(result)
}

async fn get_social_source(
    db: Extension<SocialsDb>,
    Path(raw_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let id = bson::Uuid::parse_str(raw_id.to_string()).unwrap();
    let result = SocialsDb::find_by_id(id, &db.social_sources())
        .await.unwrap();
    Json(result)
}

async fn create_social_sources(
    db: Extension<SocialsDb>,
    Json(source): Json<SocialSourceCreate>
) -> impl IntoResponse {
    let new_source = SocialSource::from(source);
    let result = SocialsDb::insert_one(new_source, db.social_sources())
        .await.unwrap();
    Json(result)
}

async fn remove_social_sources(
    db: Extension<SocialsDb>,
    Path(raw_id): Path<uuid::Uuid>
) -> impl IntoResponse {
    // TODO change to find & delete when available
    let id: bson::Uuid = bson::Uuid::parse_str(raw_id.to_string())
        .unwrap();
    let q = SocialSourceQuery {
        id: Some(id),
        ..Default::default()
    };
    let result = SocialsDb::delete_many(&q, &db.social_sources())
        .await.unwrap();
    Json(result)
}

async fn update_social_source(
    db: Extension<SocialsDb>,
    Path(raw_id): Path<uuid::Uuid>,
    Json(social_source): Json<SocialSourceCreate>
) -> impl IntoResponse {
    let id: bson::Uuid = bson::Uuid::parse_str(raw_id.to_string())
        .unwrap();

    let mut source = SocialsDb::find_by_id(id, &db.social_sources())
        .await.unwrap().unwrap();
    source.update_with(social_source);
    let res = SocialsDb::update_by_id(
        id,
        source,
        &db.social_sources()
    ).await;
    Json(res)
}
