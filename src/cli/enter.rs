use std::process::Command;

pub fn run(name: &str) {
    // start container
    Command::new("docker")
        .args(["start", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // set up container
    Command::new("docker")
        .args(["attach", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
