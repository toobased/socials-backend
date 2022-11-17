use serde::{Deserialize, Serialize};
use socials_core::social::SocialPlatform;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckBotByTokenQuery {
    pub platform: SocialPlatform,
    pub access_token: String
}
