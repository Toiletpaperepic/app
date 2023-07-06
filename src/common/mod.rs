//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

use std::{process::{Command, Stdio}, io::{self, ErrorKind, Error}, path::PathBuf};

pub(crate) fn test_run(qemu: PathBuf) -> Result<String, io::Error> {
    let child = Command::new(qemu)
        .arg("-version")
        .stdout(Stdio::piped())
        .output();

    if child.is_err() {
        return Err(Error::new(ErrorKind::NotFound, format!("Invalid QEMU Binary: {}", child.unwrap_err())));
    } else {
        let version_msg = String::from_utf8_lossy(&child.expect("Command Failed").stdout).to_string();
        info!("Found {}", version_msg.clone().as_str().replace("\n", " ").replace("\r", " "));
        return Ok(version_msg);
    }
}