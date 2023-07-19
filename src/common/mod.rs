use std::{
    io::{self, Error, ErrorKind},
    path::PathBuf,
    process::{Command, Stdio},
};

pub(crate) fn test_run(qemu: PathBuf) -> Result<String, io::Error> {
    println!("looking for qemu....");
    let child = Command::new(qemu)
        .arg("-version")
        .stdout(Stdio::piped())
        .output()
        .map_err(|err|Error::new(ErrorKind::InvalidData,format!("Invalid QEMU Binary: {}", err.to_string())));

    if child.is_err() {
        return Err(child.unwrap_err());
    } else {
        let version_msg = String::from_utf8_lossy(&child.expect("Command Failed").stdout).to_string();
        println!("Found {}", version_msg.clone().as_str().replace("\n", " ").replace("\r", " "));
        return Ok(version_msg);
    }
}