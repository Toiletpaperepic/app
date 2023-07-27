use crate::config::Config;
use crate::execute::VirtualMachines;
use std::fs;
pub(crate) mod vmid;

pub(crate) fn config(config: Config) -> VirtualMachines {
    let qemu_args = shell_words::split(
        fs::read_to_string(config.qemu_args)
            .expect("Unable to read the file: is it there?")
            .as_str(),
    )
    .unwrap();
    let virtual_machines = vmid::make(config.vnc_start_port, config.vm_slots);

    return VirtualMachines {
        qemu_args,
        qemu_bin: config.qemu_bin,
        virtual_machines,
        stream_buffer: config.stream_buffer
    };
}