use std::fs::File;
use std::io;
use std::io::ErrorKind;

pub fn errors_run() {
    let mut m = [1, 2, 3];
    let mut n = String::new();
    io::stdin().read_line(&mut n);
    println!("input {}", n);
    let k: usize = n.trim().parse().expect("input is not a number!");
    match k {
        k if k < 3 => println!("element of pos {} is {}", k, m[k]),
        _ => panic!("index pos is out of range"),
    }
    // println!("element of pos {} is {}", k, m[k]);
    // println!("divide zero: {}", 1/0);

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
