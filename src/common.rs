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

/// A data structure that can be used and understood universally,
/// holding the data needed to represent a post
#[derive(Debug)]
pub struct CommonPost {
    pub title: String,
    pub body: String,
    pub url: Option<String>,
    pub nsfw: bool,
    pub author: String,
    pub platform: Platform,
}

/// Enum that represents different social networking platforms
#[derive(Debug)]
pub enum Platform {
    Reddit,
    Lemmy,
}

/// Enum representing different ways to sort the
/// posts in a community, such as a subreddit
#[derive(Debug)]
pub enum SortMode {
    Top,
    Hot,
    New,
    Controversial,
}

/// Enum representing different ways to sort the
/// posts in a community, such as a subreddit
#[derive(Debug)]
pub enum TimeFrame {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
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
        let reponse = crate::reddit::get_posts(&ri, args.num, SortMode::Top, TimeFrame::Day).await;
        match reponse {
            Ok(posts) => {
                for post in posts {
                    crate::lemmy::submit_post(&args, post, &li).await;
                }

                // Wait before looping again
                thread::sleep(Duration::from_secs(args.wait_time));
            }
            Err(e) => {
                tracing::warn!("Retrying...");
                continue;
            }
        }
    }
}
