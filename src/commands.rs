use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lazycloud")]
#[command(about = "lazy wrapper for rclone cloud sync", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a blank config file
    Init,

    /// Show all available profiles
    #[command(alias = "ls", alias = "l")]
    List,

    /// Sync
    #[command(alias = "s", alias = "sc")]
    Sync {
        #[command(subcommand)]
        action: SyncAction,
    },

    /// Start watching a profile in the background
    Watch {
        #[command(subcommand)]
        action: SyncAction,

        /// Interval (in seconds)
        interval: u64,

        #[arg(long, hide = true, default_value_t = false)]
        __run: bool,
    },

    /// Stop a running profile in the background
    Stop {
        #[command(subcommand)]
        action: SyncAction,
    },

    /// Show all running profiles in the background
    Status,
}

#[derive(Subcommand)]
pub enum SyncAction {
    /// Sync a specific profile
    #[command(alias = "p")]
    Profile { name: String },

    /// Sync all profiles
    #[command(alias = "a")]
    All,
}
