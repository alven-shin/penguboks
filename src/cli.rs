use clap::{Parser, Subcommand};

/// Access linux development environments with ease from MacOS. Powered by Docker.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new linux environment
    Create,
    /// Enter a linux environment
    Enter,
    /// Delete an existing linux environment
    Delete,
    /// List linux environments
    List,
    /// Check health
    Health,
}
