use std::process::Command;

/// - ensure container exists
/// - start container
/// - attach to container
pub fn run(name: &str) {
    //  check for container existence
    let containers = crate::get_containers();
    if !containers.contains(name) {
        eprintln!("ERROR: failed to find container");
        eprintln!(
            "\
INFO: create container with:

      penguboks create {}\
",
            name
        );
        return;
    }

    // start container
    Command::new("docker")
        .args(["start", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // attach to container
    Command::new("docker")
        .args(["attach", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
