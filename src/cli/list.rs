use std::process::Command;

pub fn run() {
    Command::new("docker")
        .args(["ps", "--filter", "label=manager=penguboks", "--all"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
