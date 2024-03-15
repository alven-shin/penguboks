use std::process::Command;

pub fn run() {
    let name = "default";
    let prefixed_name = format!("penguboks-{}", name);

    Command::new("docker")
        .args(["container", "rm", &prefixed_name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new("docker")
        .args(["image", "rm", &prefixed_name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
