use std::{collections::HashSet, process::Command};

use clap::Parser;
use serde::{Deserialize, Serialize};

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

fn get_containers() -> HashSet<String> {
    let output = Command::new("docker")
        .args([
            "container",
            "ls",
            "--all",
            "--format",
            "json",
            "--filter",
            "label=manager=penguboks",
        ])
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8_lossy(&output);
    let mut set = HashSet::new();

    for line in output.lines() {
        let container: Container = serde_json::from_str(line).unwrap();
        set.insert(container.names);
    }

    set
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Container {
    names: String,
}
