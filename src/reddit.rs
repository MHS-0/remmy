//! Reddit stuff

use roux::{
    util::{FeedOption, RouxError},
    Me, Reddit, Submissions, Subreddit,
};

use crate::common::{get_env_var, CommonPost, Platform, SortMode, TimeFrame};

/// Struct containing needed info to do operations
/// with Reddit
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

/// Get the specified number of posts from the
/// subreddit that the program was started with,
/// sorted by the specified value
pub async fn get_posts(
    ri: &RedditInfo,
    limit: u8,
    sort: SortMode,
    time_frame: TimeFrame,
) -> Result<Vec<CommonPost>, RouxError> {
    let subreddit = Subreddit::new_oauth(&ri.subreddit, &ri.me.client);
    let feed_options = Some(
        FeedOption::new()
            .limit(limit.into())
            .period(roux::util::TimePeriod::Today),
    );

    let response = subreddit.top(limit.into(), feed_options).await?;
    tracing::info!("Successfully retrieved posts from Reddit");
    tracing::info!("posts: {:#?}", response);

    let mut posts = vec![];

    for child in response.data.children {
        let title = child.data.title.trim().to_owned();
        let author = child.data.author;
        let body = child.data.selftext.trim().to_owned();
        let nsfw = child.data.over_18;
        let url = child.data.url;

        let post = CommonPost {
            title,
            body,
            nsfw,
            url,
            author,
            platform: Platform::Reddit,
        };

        posts.push(post);
    }

    Ok(posts)
}
