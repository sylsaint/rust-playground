use std::{mem::size_of_val, cell::RefCell};
/**
 * 参考内容
 * 1. https://lelouchhe.github.io/rust_3_4
 * 2. https://www.zhihu.com/question/311028043
 */
pub fn chapter1_run() {
    // 检测可变变量重新赋值后，是指向新的地址，还是相同地址数据值变化
    let mut x = 20; 
    println!("before reassignment addr: {:p}", &x);
    x = 10; 
    println!("after reassignment addr: {:p}", &x);
    // 栈中变量是相同地址，数值变化
    let mut x1 = String::from("Hello"); 
    println!("before reassignment addr: {:p}, string: {}", &x1, x1);
    x1 = String::from("World, The quick fox jumps over the lazy dog"); 
    println!("after reassignment addr: {:p}, string: {}", &x1, x1);


    // 模式解构
    let (mut y1, mut y2) = (1, 2);
    y1 = y1 * 10;
    y2 = y2 * 10;
    println!("y1 = {}, y2 = {}", y1, y2);

    // 类型推导
    let z1 = 5u8;
    let mut v1 = Vec::new();
    v1.push(z1);
    println!("v1 is {:?}", v1);

    // 全局变量

    // 字符
    let c1 = b'A';
    println!("c1 is {}, size = {} byte", c1, size_of_val(&c1));
    let c2 = '\u{7FFF}';
    println!("c2 is {}, size = {} bytes", c2, size_of_val(&c2));

    // 整数溢出
    let o1 = 100i8;
    println!("checked add {:?}", o1.checked_add(o1));
    println!("saturing add {}", o1.saturating_add(o1));
    println!("wrapping add {}", o1.wrapping_add(o1));

    // 类型转换，必须显式处理
    let i1: i8 = 40;
    let _i2: i16 = i1 as i16;

    let p = &i1 as *const i8 as *mut i8;
    unsafe {
        let p1 = RefCell::new(*p);
        p1.replace(20);
        println!("p1 is {:?}", p1);
    }
    println!("after change i1 = {}", i1);

    let f1 = true;
    let if1 = f1 as i8;
    println!("if1 = {}", if1);
    let ic2 = c2 as usize;
    println!("ic2 = {}", ic2);
    let u1: u8 = 231;
    let uc1 = u1 as char;
    println!("uc1 is {}", uc1);

    struct Foo {};
    // 复合类型
    println!("空元组占用内存空间 () = {}", std::mem::size_of::<()>());
    println!("空结构体占用内存空间 = {}", std::mem::size_of::<Foo>());

    #[derive(Debug)]
    struct Point { 
        x: i32,
        y: i32,
        z: i32,
    }
    let x1 = Point { x: 1, y: 2, z: 3 };
    let x2 = Point { x: 3, ..x1 };
    println!("x2 is {:?}", x2);

    // tuple struct
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let color = Color(33, 33, 33);
    println!("color is {:?}", color);

    // recrusive struct
    #[derive(Debug)]
    struct Recur {
        value: i32,
        next: Option<Box<Recur>>,
    }

    let r1 = Recur { value: 10, next: None };
    let r2 = Recur { value: 20, next: Some(Box::new(r1)) };
    let r1_value = &r2.next;
    match r1_value {
         Some(v) => println!("r1 value is {:?}", v.value),
         None => (),
    }
    let r1_clone_v = r1_value;
}