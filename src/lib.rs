use clap::Parser;

mod cli;

pub fn run() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Create { name } => cli::create::run(&name),
        cli::Commands::Enter { name } => cli::enter::run(&name),
        cli::Commands::Delete { name } => cli::delete::run(&name),
        cli::Commands::List => cli::list::run(),
        cli::Commands::Health => todo!(),
    }
}
