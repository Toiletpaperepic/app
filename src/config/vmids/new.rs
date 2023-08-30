use crate::{websocket::stream::Destination, Error};
use std::process::Child;
use super::Vmid;
#[cfg(not(unix))]
use std::net::SocketAddr;
#[cfg(unix)]
use std::path::PathBuf;

//only for testing
pub(crate) fn vmid(destination: Destination, vmids: usize) -> Result<Vec<Vmid>, Error> {
    let mut vec: Vec<Vmid> = Vec::new();
    let mut qenu_port: u16 = 9500;

    for vmid_number in 0..vmids {
        let destination;
        cfg_if::cfg_if! {
            if #[cfg(unix)] { 
                destination = Destination::Unix(PathBuf::from(format!("/tmp/vmp{}", vmid_number)))
            } else {
                destination = Destination::Tcp(SocketAddr::from(([127, 0, 0, 1], qenu_port)))
            }
        }

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