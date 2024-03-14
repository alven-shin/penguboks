use clap::Parser;

mod cli;

pub fn run() {
    let cli = cli::Cli::parse();
}
