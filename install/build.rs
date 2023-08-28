use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=../packup");
    let embeddir = fs::read_to_string("../packup/index.txt").unwrap();
    println!("cargo:rustc-env=EMDED={}", embeddir)
}