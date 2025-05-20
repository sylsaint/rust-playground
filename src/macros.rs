use attr_macro::route;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use show_macro::show_streams;

#[macro_export]
macro_rules! cvec {
    ($($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        } 
    };
}

#[show_streams]
fn invoke1() {}

#[derive(HelloMacro)]
struct Pancakes;

#[route(GET, "/")]
pub fn index(input: String) -> String {
    input
}

macro_rules! S {
    ($e:expr) => {
        5 * $e
    };
}


pub fn macros_run() {
    let v: Vec<u32> = cvec![1,2,3];
    println!("vector is {:?}", v);
    Pancakes::hello_macro();
    let w = S!(3+4);
    println!("hello, {}", w);
}