//! Stuff related to the cli options

use clap::Parser;

/// Command-Line interface configuration happens here
#[derive(Parser)]
pub struct Args {
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
}
