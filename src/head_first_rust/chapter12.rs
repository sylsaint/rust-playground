fn get_less<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32
where 'a: 'b
{
    y
}

fn get_str(s: &String) -> &str {
    println! ("call fn {}", s);
    "hello world"
}

pub fn chapter12_run() {
    // 生命周期标记
    let x = 20i32;
    {
        let y = 30i32;
        let z = get_less(&y, &x);
        println!("z is {}", z);

    }
    let s = String::from("hehe");
    // 这个无法编译通过，因为s的生命周期不是'static
    // let x: &'static str = get_str(&s);
}
