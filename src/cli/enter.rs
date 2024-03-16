use std::process::Command;

pub fn run(name: &str) {
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

    // set up container
    Command::new("docker")
        .args(["attach", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
