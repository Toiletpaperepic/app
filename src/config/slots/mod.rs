use crate::config::Config;
use crate::execute::VirtualMachines;
use crate::test_run;
use std::fs;
pub(crate) mod vmid;

pub(crate) fn config(config: Config) -> VirtualMachines {
    let qemu_args = shell_words::split(
        fs::read_to_string("config/qemu.args")
            .expect("Unable to read the file: is it there?")
            .as_str(),
    )
    .unwrap();
    let virtual_machines = vmid::make(config.vnc_start_port, config.vm_slots);
    let qemu_bin = config.qemu_bin.clone();
    let version_msg = test_run(config.qemu_bin.clone()).unwrap();

    return VirtualMachines {
        qemu_args,
        qemu_bin,
        version_msg,
        virtual_machines,
    };
}
