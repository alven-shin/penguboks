use clap::Parser;

mod cli;

pub fn run() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Create => cli::create::run(),
        cli::Commands::Enter => cli::enter::run(),
        cli::Commands::Delete => cli::delete::run(),
        cli::Commands::List => cli::list::run(),
        cli::Commands::Health => todo!(),
    }
}
