fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

fn call_with_closure<F>(some_closure: F) -> i32
where
    F: Fn(i32) -> i32,
{
    some_closure(1)
}

fn static_dispatch<F>(closure: &F)
//  这里是泛型参数。对于每个不同类型的参数，编译器将会生成不同版本的函数
where
    F: Fn(i32) -> i32,
{
    println!("static dispatch {}", closure(42));
}

fn dynamic_dispatch(closure: &dyn Fn(i32) -> i32)
// 这里是 `trait object``Box<Fn(i32)->i32>`也算`trait object`。
{
    println!("dynamic dispatch {}", closure(42));
}

pub fn chapter22_run() {
    let add = |a: i32, b: i32| -> i32 { a + b };
    let x = add(1, 2);
    println!("x is {}", x);

    let f = make_adder(3);

    println!("{}", f(1)); // 4
    println!("{}", f(10)); // 13
    let answer = call_with_closure(|x| x + 2);
    println!("{}", answer);
    // 同一个变量绑定了两次
    let mut closure = |x: i32| -> i32 { x + 2 };
    // closure = |x: i32| -> i32 { x - 2 };
    println!("{}", closure(10));
    let closure1 = | x | x * 2;
    let closure2 = | x | x * 3;
    fn function_ptr(x: i32)->i32 { x * 4 };

    static_dispatch(&closure1);
    static_dispatch(&closure2);
    static_dispatch(&function_ptr); // 普通`fn`函数也实现了`Fn trait`，它可以与此参数类型匹配。`fn`不可以捕获外部变量

    dynamic_dispatch(&closure1);
    dynamic_dispatch(&closure2);
    dynamic_dispatch(&function_ptr);
}
