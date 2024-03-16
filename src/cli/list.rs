use std::process::Command;

/// - list all containers that have the penguboks label
pub fn run() {
    Command::new("docker")
        .args(["ps", "--filter", "label=manager=penguboks", "--all"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
