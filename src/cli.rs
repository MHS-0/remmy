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
}
