use crate::execute::VirtualMachines;
use crate::config::Config;
use crate::setup::VmidConfig;
use std::fs;
pub(crate) mod addmutex;
pub(crate) mod preload;

pub(crate) fn config(config: Config) -> VirtualMachines {
    let virtual_machines: VmidConfig = serde_json::from_str(
        fs::read_to_string(config.qemu_args.clone())
            .expect("Unable to read the file: is it there?")
            .as_str(),
    )
    .unwrap();

    info!("deserialized = {:#?}", virtual_machines);

    return VirtualMachines {
        virtual_machines: addmutex::run(virtual_machines.0),
        config
    };
}