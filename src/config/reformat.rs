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
    let v: Vec<&str> = qemu_args.trim().split_terminator('"').collect();
    let mut even = false;

    for v2 in v {
        if even {
            vec.push(v2.to_owned());
            even = false;
            continue;
        }
        else {
            for v3 in v2.split(' ') {
                if v3.replace("\t", "").is_empty() {
                    //
                } else {
                    vec.push(v3.to_owned());
                }
            }
        }
        even = true;
    }

    return vec;
}