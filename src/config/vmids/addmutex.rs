use std::sync::{Mutex, Arc};
use super::preload::Vmid;

pub(crate) fn run(vec_vmid: Vec<Vmid>) -> Vec<Arc<Mutex<Vmid>>> {
    let mut vec: Vec<Arc<Mutex<Vmid>>> = Vec::new();
    for vmid in vec_vmid {
        vec.push(Arc::new(Mutex::new(vmid)))
    }
    return vec;
}