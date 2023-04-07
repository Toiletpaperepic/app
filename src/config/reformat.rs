pub(crate) fn split_argument(qemu_args: Vec<String>) -> Vec<std::string::String>{
    let mut vec: Vec<String> = Vec::new();

    for arg in qemu_args {
        let v: Vec<&str> = arg.split(' ').collect();

        for v2 in v {
            vec.push(v2.to_owned())
        }
        
    }
    return vec;
}