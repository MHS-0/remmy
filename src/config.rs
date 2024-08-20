use serde::{Deserialize, Serialize};

use crate::{common::BotMode, lemmy::LemmyInfo, reddit::RedditInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub mode: BotMode,
    pub reddit: RedditConfig,
    pub lemmy: LemmyConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RedditConfig {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub subreddit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LemmyConfig {
    pub instance: String,
    pub community: String,
    pub username: String,
    pub password: String,
}
