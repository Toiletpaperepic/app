// use std::io::{stdin, BufRead};

// pub(crate) fn new() {
//     let mut s_buffer = String::new();
//     let mut stdin = stdin().lock();

//     loop {
//         s_buffer.clear();
//         stdin.read_line(&mut s_buffer).unwrap();
//         let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");

//         println!("{:?}", line);
    
//         if line.contains("stop") {
//             break
//         }
//     }
// }