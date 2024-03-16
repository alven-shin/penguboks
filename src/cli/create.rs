use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

/// - ensure container does not already exist
/// - create image for container to save disk space and change user
/// - label the images and directories
/// - create container and mount directories
pub fn run(name: &str) {
    // check for container existence
    let containers = crate::get_containers();
    if containers.contains(name) {
        eprintln!("ERROR: container already exists");
        eprintln!(
            "\
INFO: enter container with:

      penguboks enter {}\
",
            name
        );
        return;
    }

    let image_name = format!("penguboks-{}", name);

    // username
    let whoami = Command::new("whoami").output().unwrap().stdout;
    let user = String::from_utf8_lossy(&whoami);

    // home directory
    let host_home = dirs::home_dir().unwrap();
    let mut container_home = PathBuf::new();
    container_home.push("/home");
    container_home.push(host_home.file_name().unwrap());

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

RUN useradd --no-log-init --user-group --create-home --home-dir {home} --shell /bin/bash {username} \
    && echo '{username} ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
USER {username}
WORKDIR {home}
",
        home = container_home.to_string_lossy(),
        username = user.trim(),
    )
    .unwrap();
    build.wait().unwrap();

    // create container
    let mut create = Command::new("docker");
    create.args([
        "create",
        "--interactive",
        "--tty",
        "--label",
        "manager=penguboks",
        "--hostname",
        "penguboks",
        "--name",
        name,
    ]);

    // document folder
    let document_src = host_home.join("Documents");
    let document_dst = container_home.join("Documents");
    if document_src.is_dir() {
        create.args([
            "--mount",
            &format!(
                "type=bind,source={},target={}",
                document_src.to_string_lossy(),
                document_dst.to_string_lossy()
            ),
        ]);
    }

    // downloads folder
    let downloads_src = host_home.join("Downloads");
    let downloads_dst = container_home.join("Downloads");
    if downloads_src.is_dir() {
        create.args([
            "--mount",
            &format!(
                "type=bind,source={},target={}",
                downloads_src.to_string_lossy(),
                downloads_dst.to_string_lossy()
            ),
        ]);
    }

    // public folder
    let public_src = host_home.join("Public");
    let public_dst = container_home.join("Public");
    if public_src.is_dir() {
        create.args([
            "--mount",
            &format!(
                "type=bind,source={},target={}",
                public_src.to_string_lossy(),
                public_dst.to_string_lossy()
            ),
        ]);
    }

    // config folder
    let config_src = host_home.join(".config");
    let config_dst = container_home.join(".config");
    if config_src.is_dir() {
        create.args([
            "--mount",
            &format!(
                "type=bind,source={},target={}",
                config_src.to_string_lossy(),
                config_dst.to_string_lossy()
            ),
        ]);
    }

    create.arg(&image_name);
    create.spawn().unwrap().wait().unwrap();
}
