use std::fs::File;
use std::io::Read;

pub fn read_run() {
    let f = File::open("/Users/yonglusun/codes/rust-playground/src/read.rs");
    if f.is_err() {
        println!("file is not exist.");
        return;
    }
    let mut f = f.unwrap();
    let mut content = String::new();
    let result = f.read_to_string(&mut content);
    if result.is_err() {
        println! ("read file error.");
        return;
    }
    println! ("{}", result.unwrap());
}
