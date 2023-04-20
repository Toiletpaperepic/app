use std::{fs::File, io::Write};

pub(crate) fn make(mut qenu_port: i32, vm_slots: i32) -> Vec<i32> {
    let mut token_list = File::create("config/token.list").expect("failed to create the log file.");
    let mut vec: Vec<i32> = Vec::new();
    let stop = qenu_port + vm_slots;
    
    loop {
        vec.push(qenu_port);
        qenu_port += 1;

        if stop == qenu_port {
            break;
        }
    }
    return vec;
}
