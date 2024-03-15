pub mod create;
pub mod delete;
pub mod enter;
pub mod list;

use clap::{Parser, Subcommand};

/// Access linux development environments with ease from MacOS. Powered by Docker.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new linux environment
    Create {
        #[arg(default_value = "default")]
        name: String,
    },
    /// Enter a linux environment
    Enter {
        #[arg(default_value = "default")]
        name: String,
    },
    /// Delete an existing linux environment
    Delete {
        #[arg(default_value = "default")]
        name: String,
    },
    /// List linux environments
    List,
    /// Check health
    Health,
}
