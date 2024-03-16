use std::process::Command;

/// - ensure container exists
/// - remove container
/// - remove image
pub fn run(name: &str) {
    // check for container existence
    let containers = crate::get_containers();
    if !containers.contains(name) {
        eprintln!("ERROR: failed to find container");
        return;
    }

    Command::new("docker")
        .args(["container", "rm", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let image_name = format!("penguboks-{}", name);
    Command::new("docker")
        .args(["image", "rm", &image_name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
