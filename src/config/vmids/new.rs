use crate::{websocket::stream::Destination, Error};
use std::net::SocketAddr;
use std::process::Child;
use super::Vmid;
#[cfg(unix)]
use std::path::PathBuf;

pub(crate) enum DestinationOption{
    #[cfg(unix)]
    Unix(PathBuf),
    Tcp(u16)
}

//only for testing
pub(crate) fn vmid(destination_option: DestinationOption, vmids: usize) -> Result<Vec<Vmid>, Error> {
    let mut vec: Vec<Vmid> = Vec::new();

    for vmid_number in 0..vmids {
        let destination = match destination_option {
            #[cfg(unix)]
            DestinationOption::Unix(ref path) => Destination::Unix(path.join(format!("vmu{}", vmid_number))),
            DestinationOption::Tcp(port) => Destination::Tcp(SocketAddr::from(([127, 0, 0, 1], port + <usize as TryInto<u16>>::try_into(vmid_number).unwrap()))),
        };

        let vmid = Vmid {
            vmid_number,
            destination,
            child: None::<Child>,
            qemu_arg: vec![],
            password: None,
            name: "No Name".to_string(),
            path: None,
        };
        info!("preloading.... {:#?}", vmid);
        vec.push(vmid);
    }

    Ok(vec)
}

#[test]
fn test() -> Result<(), Error> {
    let vmid = vmid(DestinationOption::Tcp(9500), 4)?;

    assert_eq!(vmid.len(), 4);

    //test 0
    // assert_eq!(vmid[0].port, 5900);
    assert_eq!(vmid[0].vmid_number, 0);

    //test 4
    // assert_eq!(vmid[3].port, 5903);
    assert_eq!(vmid[3].vmid_number, 3);
    Ok(())
}