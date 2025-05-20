fn echo(x: &String) -> &String {
    x
}

fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn add2((x, y): (i32, i32)) -> i32 {
    x + y
}

pub fn chapter4_run() {
    let echo1 = echo;
    let x = String::from("hello");
    println!("echo1 {} is {}", x, echo1(&x));
    println!("echo {} is {}", x, echo(&x));
    let mut add: fn((i32, i32)) -> i32 = add1 as fn((i32, i32)) -> i32;
    add = add2;
    println!("10 + 11 = {}", add((10, 11)));
}