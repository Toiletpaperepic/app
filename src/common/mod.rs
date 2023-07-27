use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub(crate) fn test_run(qemu: PathBuf) {
    info!("looking for qemu....");
    let child = Command::new(qemu)
        .arg("-version")
        .stdout(Stdio::piped())
        .output();

    if child.is_err() {
        panic!("Invalid QEMU Binary: {}", child.unwrap_err());
    } else {
        let version_msg = String::from_utf8_lossy(&child.expect("Command Failed").stdout).to_string();
        info!("Found {}", version_msg.clone().as_str().replace("\n", " ").replace("\r", " "));
    }
}