//! Stuff that's independent from Reddit and Lemmy or common
//! to both

use std::{thread, time::Duration};

use lemmy_client::lemmy_api_common::post::CreatePost;
use roux::{util::FeedOption, Subreddit};

use crate::{cli::Args, lemmy::LemmyInfo, reddit::RedditInfo};

/// Get environment variable with the given key and panic
/// if it isn't found.
pub fn get_env_var(key: &str) -> String {
    std::env::var(key).expect(&format!("{key} environment variable wasn't set"))
}

/// What should the program do?
// TODO: We could expand this in the future
// to do different things
#[derive(Default)]
pub enum BotMode {
    /// Crosspost from Reddit to Lemmy
    #[default]
    CrossRTL,
    /// Crosspost from Lemmy to Reddit
    CrossLTR,
}

/// Start a loop to cross post Reddit stuff to
/// Lemmy.
pub async fn start_cross_rtl_loop(args: Args, ri: RedditInfo, li: LemmyInfo) -> ! {
    // Ctrl+c handling happens here
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tracing::info!("Interrupt signal received, stopping...");
        std::process::exit(0);
    });

    tracing::info!(
        "Starting cross posting loop from \
        Reddit to Lemmy"
    );

    loop {
        let subreddit = Subreddit::new_oauth(&ri.subreddit, &ri.me.client);
        let limit = args.num;
        let feed_options = Some(
            FeedOption::new()
                .limit(limit.into())
                .period(roux::util::TimePeriod::Today),
        );

        match subreddit.top(limit.into(), feed_options).await {
            Ok(data) => {
                tracing::info!("Successfully retrieved today's top posts");
                tracing::info!("{:#?}", data);

                for child in data.data.children {
                    let title = child.data.title.trim().to_owned();
                    let author = child.data.author;
                    let body = child.data.selftext.trim().to_owned()
                        + &format!(
                            "\n\n\
                        This post was authored by: /u/{author} on Reddit\n      \
                        If you liked this post, give them a visit!"
                        );
                    let nsfw_flag = child.data.over_18;
                    let url = child.data.url;

                    let post = CreatePost {
                        community_id: li.community_id,
                        name: title.clone(),
                        body: Some(body),
                        nsfw: Some(nsfw_flag),
                        url,
                        ..Default::default()
                    };
                    if args.dry_run {
                        tracing::info!("{post:#?}");
                    } else {
                        loop {
                            match li.me.create_post(post.clone()).await {
                                Ok(post_info) => {
                                    tracing::info!(
                                        "Successfully posted on Lemmy!\n\
                                    Post's info: {post_info:#?}"
                                    );
                                    break;
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "\
                                    Error when posting Reddit post with title: \"{title}\".\n\
                                    Error encountered: {e} \n\
                                    Retrying after {} seconds...",
                                        args.retry_time
                                    );
                                    thread::sleep(Duration::from_secs(args.retry_time));
                                }
                            };
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed getting Reddit data: {e}. Retrying...");
                continue;
            }
        }
        // Wait for a full day before looping again
        thread::sleep(Duration::from_secs(args.wait_time));
    }
}
