// Don't worry about unused stuff for now
#![allow(unused)]

use clap::Parser;
use cli::Args;
use common::{start_cross_rtl_loop, BotMode};
use config::Config;
use lemmy::initialize_lemmy_client;
use reddit::initialize_reddit_client;

mod cli;
mod common;
mod config;
mod lemmy;
mod reddit;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let args = Args::parse();

    let config: Config = serde_yaml::from_slice(
        &std::fs::read(args.config.clone()).expect("Error Reading given config file"),
    )
    .expect("Error deserializing given config file");

    let reddit_info = initialize_reddit_client(config.clone()).await;
    let lemmy_info = initialize_lemmy_client(config.clone()).await;

    // TODO: In the future, we could decide to do different things
    // instead depending on the config. For example, cross post from
    // Lemmy to Reddit instead, use other platforms completely, or do
    // things besides cross posting such as moderation, etc.
    let bot_mode = config.mode;

    match bot_mode {
        BotMode::CrossRTL => start_cross_rtl_loop(args, reddit_info, lemmy_info).await,
        BotMode::CrossLTR => (),
    }
}
