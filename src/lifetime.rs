use std::ops::Drop;

struct D(i32);

impl Drop for D {
    fn drop(&mut self) {
        println!("value has been droped, {}", self.0);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}

pub fn lifetime_run() {
    let INITIAL = 10;
    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("r: {}", r);
    let _x = D(1);
    println!("construct 1");
    {
        let _y = D(2);
        println!("construct 2");
        println!("exit inner scope");
    }
    println!("exit main function");
}
