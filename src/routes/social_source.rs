use axum::{
    extract::Path,
    routing::get,
    response::IntoResponse,
    Json, Router, Extension
};

use socials_core::{db::{SocialsDb, DummyQuery }, social::source::{SocialSourceQuery, SocialSource, SocialSourceCreate}};

pub fn social_source_router() -> Router {
    let r: Router = Router::new()
        .route("/social_sources/",
            get(get_social_sources)
            .post(create_social_sources)
        )
        .route("/social_sources/:source_id",
            get(get_social_source)
            .delete(remove_social_sources)
            .patch(update_social_source)
        );
    r
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
