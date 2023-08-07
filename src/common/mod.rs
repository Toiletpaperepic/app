use rocket::{response::Redirect, State};
use crate::execute::VirtualMachines;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[get("/")]
pub(crate) fn index(vms: &State<VirtualMachines>) -> Redirect {
    if vms.args.setup {
        Redirect::to(uri!("/admin/index.html"))
    } else {
        Redirect::to(uri!("/home/index.html"))
    }
}

#[get("/favicon.ico")]
pub(crate) fn favicon() -> Redirect {
    Redirect::to(uri!("/home/favicon.ico"))
}

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