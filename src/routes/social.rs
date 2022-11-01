use axum::{
    routing::get,
    response::IntoResponse,
    Json, Router, Extension
};

use socials_core::{db::SocialsDb, social::post::SocialPost};

use self::query::GetPostByUrlQuery;

pub mod query;

pub fn social_router() -> Router {
    let r: Router = Router::new()
        .route("/social/get_post_by_url",
            get(dummy)
            .post(get_post_by_url)
        );
    r
}

async fn dummy () -> impl IntoResponse { Json("") }

async fn get_post_by_url(
    _db: Extension<SocialsDb>,
    Json(data): Json<GetPostByUrlQuery>
) -> impl IntoResponse {
    let res = SocialPost::get_post_by_url(&data.platform, &data.url)
        .await;
    Json(res)
}
