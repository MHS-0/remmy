//! Reddit stuff

use roux::{Me, Reddit};

use crate::common::get_env_var;

/// Struct containing a validated Reddit client,
/// and the subreddit of interest
pub struct RedditInfo {
    /// An authorized Reddit client
    pub me: Me,

    /// Name of the subreddit that we do stuff with
    pub subreddit: String,
}

impl RedditInfo {
    /// Create a new [RedditInfo] instance with the given
    /// Reddit client ([roux::Me]) and the subreddit of interest
    pub fn new(me: Me, subreddit: String) -> Self {
        Self { me, subreddit }
    }
}

/// Get the required veraibles from the environment
/// and setup a usable reddit client.
///
/// Panics if the variables are not given or correct,
/// or if there are connection issues.
pub async fn initialize_reddit_client() -> RedditInfo {
    tracing::info!("Setting up and validating the Reddit client...");

    // TODO: We could do this by using a serde serialized yaml file too.
    // Get Reddit related env variables
    let reddit_client_id = get_env_var("REDDIT_CLIENT_ID");
    let reddit_client_secret = get_env_var("REDDIT_CLIENT_SECRET");
    let reddit_username = get_env_var("REDDIT_USERNAME");
    let reddit_password = get_env_var("REDDIT_PASSWORD");
    let subreddit = get_env_var("SUBREDDIT");

    // Setup the Reddit client
    let reddit_client = Reddit::new("remmy:roux:v2", &reddit_client_id, &reddit_client_secret)
        .username(&reddit_username)
        .password(&reddit_password)
        .login()
        .await;

    if let Err(e) = reddit_client {
        tracing::error!("Error logging in to Reddit: {e}");
        std::process::exit(1);
    };

    RedditInfo::new(reddit_client.unwrap(), subreddit)
}
