use std::{ops::{Drop, Deref}, borrow::Borrow};

struct S {
    v: i32,
}

impl Drop for S {
    fn drop(&mut self) {
        println!("destructor for {}", self.v);
    }
}

fn test<'a>(arg: &'a S) -> &'a i32 {
    &arg.v
}

fn test1<'a, 'b>(arg: &'a S) -> &'b i32 where 'a:'b {
    &arg.v
}

fn joint() {
    let s = Box::new(String::from("hello"));
    let t = &*s;
    println!("both are s: {}, t: {}", s, t);
}

fn seperate() {
    let s = Box::new(String::from("hello"));
    let tmp = &{*s};
    let t = &tmp;
    println!("both are s: {}, t: {}", tmp, t);
}

fn r() -> &'static str {
    static S: &str = "hehe";
    return S;
}

pub fn borrow_run() {
    let v1 = 1_i32;
    let mut var = 0_i32;
    var = v1;
    println!("var is {}", var);
    {
        let p1 = &mut var;
        *p1 = 10;
    }
    println!("var is {}", var);
    {
        let temp = 3_i32;
        let mut p2 = &var;
        println!("p2 is {}", p2);
        p2 = &temp;
        println!("p2 is {}", p2);
    }

    let s1 = S { v: 10 };
    let mut s2 = S { v: 12 };
    s2 = s1;
    println!("s2 value {}", s2.v);


    // 多次借用
    let mut x = 1_i32;
    let p1 = &x;
    let p2 = &x;
    let mut x1 = &x;
    println!("p1 is {:p}, p2 is {:p}, x is {:p}", p1, p2, &x);
    println!("x1 is {:p}", x1);
    joint();
    seperate();
    let s1 = r();
    println!("s1 is {}", s1);


    // 模式匹配不会自动解引用
    let s = String::new();
    match s.borrow() {
        "" => { println!("an empty string"); }
        _ => {}
    }
}