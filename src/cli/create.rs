use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn run(name: &str) {
    let image_name = format!("penguboks-{}", name);

    let whoami = Command::new("whoami").output().unwrap().stdout;
    let user = String::from_utf8_lossy(&whoami);

    // build image
    let mut build = Command::new("docker")
        .args([
            "build",
            "--label",
            "manager=penguboks",
            "--tag",
            &image_name,
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

RUN useradd --no-log-init --user-group --create-home --shell /bin/bash {username} \
    && echo '{username} ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
USER {username}
WORKDIR /home/{username}
",
        username = user.trim()
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
            "--hostname",
            "penguboks",
            "--name",
            name,
            &image_name,
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
