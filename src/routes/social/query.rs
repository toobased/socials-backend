use serde::{Deserialize, Serialize};
use socials_core::social::SocialPlatform;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPostByUrlQuery {
    pub platform: SocialPlatform,
    pub url: String
}
