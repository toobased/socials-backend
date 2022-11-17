use axum::{
    // extract::Path,
    routing::{get, post},
    response::IntoResponse,
    Json, Router, Extension, extract::{Query, Path}
};

use socials_core::{db::SocialsDb, bots::{query::BotQuery, BotCreate, Bot, BotUpdate}};

use self::query::CheckBotByTokenQuery;

// local modules
pub mod query;

pub fn bots_router() -> Router {
    let r: Router = Router::new()
        .route("/bots/",
            get(get_bots)
            .post(create_bot)
        )
        .route("/bots/check_by_token",
            post(check_by_token)
        )
        .route("/bots/:id",
            get(get_bot)
            .delete(remove_bot)
            .patch(update_bot)
        );
    r
}

async fn get_bots(
    db: Extension<SocialsDb>,
    query: Option<Query<BotQuery>>
) -> impl IntoResponse {
    let Query(q) = query.unwrap_or_default();
    let result = SocialsDb::find(&q, &db.bots())
        .await.unwrap();
    Json(result)
}

async fn get_bot(
    db: Extension<SocialsDb>,
    Path(raw_id): Path<uuid::Uuid>
) -> impl IntoResponse {
    let id: bson::Uuid = bson::Uuid::parse_str(raw_id.to_string()).unwrap();
    let result = SocialsDb::find_by_id(id, &db.bots())
        .await.unwrap();
    Json(result)
}

async fn remove_bot(
    db: Extension<SocialsDb>,
    Path(raw_id): Path<uuid::Uuid>
) -> impl IntoResponse {
    let id: bson::Uuid = bson::Uuid::parse_str(raw_id.to_string()).unwrap();
    let mut q = BotQuery::new();
    q.id = Some(id);
    let result = SocialsDb::delete_many(&q, &db.bots())
        .await.unwrap();
    Json(result)
}

async fn update_bot(
    db: Extension<SocialsDb>,
    Path(raw_id): Path<uuid::Uuid>,
    Json(upd_bot): Json<BotUpdate>
) -> impl IntoResponse {
    let id: bson::Uuid = bson::Uuid::parse_str(raw_id.to_string()).unwrap();
    let mut bot = SocialsDb::find_by_id(id, &db.bots()).await.unwrap().unwrap();
    bot.update_with(upd_bot).update_db(&db).await.unwrap();
    Json(bot)
}

async fn create_bot(
    db: Extension<SocialsDb>,
    Json(raw_bot): Json<BotCreate>
) -> impl IntoResponse {
    let b = Bot::create_from(&db, raw_bot)
        .await.unwrap();
    let res = SocialsDb::insert_one(b, db.bots()).await.unwrap();
    Json(res)
}

async fn check_by_token(
    Json(query): Json<CheckBotByTokenQuery>
) -> impl IntoResponse {
    let res = Bot::fetch_by_access_token(
        query.platform,
        &query.access_token
    ).await;
    Json(res)
}
