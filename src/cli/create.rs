use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn run() {
    let name = "default";
    let prefixed_name = format!("penguboks-{}", name);

    // build image
    let mut build = Command::new("docker")
        .args([
            "build",
            "--label",
            "manager=penguboks",
            "--tag",
            &prefixed_name,
            "-",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(
        build.stdin.as_mut().unwrap(),
        "
FROM fedora:latest

RUN dnf install -y pulseaudio \
    && rm -rf /var/cache/dnf

RUN useradd --no-log-init --user-group --create-home --shell /bin/bash penguboks \
    && echo 'penguboks ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
USER penguboks
WORKDIR /home/penguboks
",
    )
    .unwrap();
    build.wait().unwrap();

    // create container
    Command::new("docker")
        .args([
            "create",
            "--interactive",
            "--tty",
            "--label",
            "manager=penguboks",
            "--name",
            &prefixed_name,
            &prefixed_name,
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
