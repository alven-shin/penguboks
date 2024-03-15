use std::process::Command;

pub fn run() {
    let name = "default";
    let container_name = format!("penguboks-{}", name);

    // start container
    Command::new("docker")
        .args(["start", &container_name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // set up container
    Command::new("docker")
        .args(["attach", &container_name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
