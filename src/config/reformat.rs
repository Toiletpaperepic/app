//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

pub(crate) fn split_argument(qemu_args: String) -> Vec<String>{
    let mut vec: Vec<String> = Vec::new();
    let v: Vec<&str> = qemu_args.split(' ').collect();

    for v2 in v {
        vec.push(v2.to_owned())
    }

    return vec;
}