#![feature(get_type_id)]

use std::any::Any;
use std::fmt::Display;

fn log<T: Any + Display>(value: &T) {
    let value_any = value as &dyn Any;

    if let Some(s) = value_any.downcast_ref::<String>() {
        println!("String: {}", s);
    } else if let Some(i) = value_any.downcast_ref::<i32>() {
        println!("i32: {}", i);
    } else {
        let type_id = value_any.type_id();
        println!("unknown type {:? }: {}", type_id, value);
    }
}

fn do_work<T: Any + Display>(value: &T) {
    log(value);
}

pub fn chapter26_run() {
    let my_string = "Hello World".to_string();
    do_work(&my_string);

    let my_i32: i32 = 100;
    do_work(&my_i32);

    let my_char: char = '❤';
    do_work(&my_char);
}
