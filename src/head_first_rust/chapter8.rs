fn panic_with_i32(x: i32) -> ! {
    let s = format!("abc, {}", x);
    panic!("{}", s);
}

pub fn chapter8_run() {
    fn call_fn<T, F: Fn(i32) -> T>(f: F, arg: i32) -> T {
        f(arg)
    }
    // 如果不把 ! 当成一个类型，那么下面这句话会出现编译错误，因为只有类型才能替换类型参数
    // call_fn(panic_with_i32, 0);

    use std::mem::size_of;
    println!("size of isize              : {}", size_of::<isize>());
    println!(
        "size of Option<isize>      : {}",
        size_of::<Option<isize>>()
    );
    println!("size of &isize             : {}", size_of::<&isize>());
    println!("size of Box<isize>         : {}", size_of::<Box<isize>>());
    println!(
        "size of Option<&isize>      : {}",
        size_of::<Option<&isize>>()
    );
    println!(
        "size of Option<Box<isize>>  : {}",
        size_of::<Option<Box<isize>>>()
    );
    println!("size of *const isize      : {}", size_of::<*const isize>());
    println!(
        "size of Option<*const isize> : {}",
        size_of::<Option<*const isize>>()
    );
}
