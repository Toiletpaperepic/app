use std::process::Child;
use super::Vmid;

//only for testing
pub(crate) fn vmid(mut qenu_port: u16, vmids: usize) -> Vec<Vmid> {
    let mut vec: Vec<Vmid> = Vec::new();

    for vmid_number in 0..vmids {
        let vmid = Vmid {
            vmid_number,
            port: qenu_port,
            child: None::<Child>,
            qemu_arg: vec![],
            name: "No Name".to_string(),
        };
        info!("preloading.... {:#?}", vmid);
        vec.push(vmid);
        qenu_port += 1;
    }

    return vec;
}

#[test]
fn test() {
    let vmid = vmid(5900,4);

    assert_eq!(vmid.len(), 4);

    //test 0
    assert_eq!(vmid[0].port, 5900);
    assert_eq!(vmid[0].vmid_number, 0);

    //test 4
    assert_eq!(vmid[3].port, 5903);
    assert_eq!(vmid[3].vmid_number, 3);
}