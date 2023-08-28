use crate::{websocket::stream::Destination, Error};
use std::{process::Child, path::PathBuf, net::SocketAddr};
use super::Vmid;

//only for testing
pub(crate) fn vmid(vmids: usize) -> Result<Vec<Vmid>, Error> {
    let mut vec: Vec<Vmid> = Vec::new();
    let id: usize = 0;
    let qenu_port: u16 = 9500;

    for vmid_number in 0..vmids {
        let destination = if cfg!(unix) {
            Destination::Unix(PathBuf::from("/tmp/"))
        } else if cfg!(windows) {
            Destination::Tcp(SocketAddr::from(([127, 0, 0, 1], qenu_port)))
        } else {
            Destination::Tcp(SocketAddr::from(([127, 0, 0, 1], qenu_port)))
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
        qenu_port += 1;
    }

    Ok(vec)
}

#[test]
fn test() -> Result<(), Error> {
    let vmid = vmid(5900)?;

    assert_eq!(vmid.len(), 4);

    //test 0
    // assert_eq!(vmid[0].port, 5900);
    assert_eq!(vmid[0].vmid_number, 0);

    //test 4
    // assert_eq!(vmid[3].port, 5903);
    assert_eq!(vmid[3].vmid_number, 3);
    Ok(())
}