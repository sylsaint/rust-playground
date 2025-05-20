pub fn functions_run() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let ans = do_twice(add_one, 10);
    println!("answer is {}", ans);
}