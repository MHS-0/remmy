//! Stuff related to the cli options

use std::path::PathBuf;

use clap::Parser;

use crate::common::{SortMode, TimeFrame};

/// Command-Line interface for Remmy
/// A Reddit to Lemmy bot
#[derive(Parser, Debug)]
pub struct Args {
    /// Path to config file
    #[arg(short, long)]
    pub config: PathBuf,

    /// Number of posts to make
    #[arg(short, long, default_value_t = 5)]
    pub num: u8,

    /// Whether to actually post anything or not
    /// Useful for seeing what posts WILL be made if
    /// you were torun this program
    #[arg(short, long)]
    pub dry_run: bool,

    /// How long should I wait before posting again? (in seconds)
    /// Default is 86400 seconds or 1 day
    #[arg(short, long, default_value_t = 86400)]
    pub wait_time: u64,

    /// How long should I wait before retrying to post after an error? (in seconds)
    #[arg(short, long, default_value_t = 5)]
    pub retry_time: u64,

    /// How to sort the posts of the source platform?
    #[arg(value_enum, short, long, default_value_t = SortMode::Top)]
    pub sorting: SortMode,

    /// what time frame to use for the posts of the source platform?
    #[arg(value_enum, short, long, default_value_t = TimeFrame::Day)]
    pub time_frame: TimeFrame,
}
