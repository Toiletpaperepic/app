use std::{process::{Command, Stdio}, io};

pub(crate) fn test_run(qemu: String) -> io::Result<String> {
    let child = Command::new(qemu)
        .arg("-version")
        .stdout(Stdio::piped())
        .output();

    if child.is_err() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Invalid QEMU Binary: {}", child.unwrap_err())));
    }
    else {
        let version_msg = String::from_utf8_lossy(&child.expect("Command Failed").stdout).to_string();
        info!("Found {}", version_msg.clone().as_str().replace("\n", " ").replace("\r", " "));
        return Ok(version_msg);
    }
}