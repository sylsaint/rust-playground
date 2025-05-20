pub fn chapter5_run() {
    trait Shape {
        fn area(&self) -> f64;
    }
    trait Round {
        fn get_radius(&self) -> f64;
    }
    impl Shape for dyn Round {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.get_radius() * self.get_radius()
        }
    }
    struct Circle {
        radius: f64,
    }
    impl Round for Circle {
        fn get_radius(&self) -> f64 {
            self.radius
        }
    }
    let c = Circle { radius: 10.0 };
    println!("circle area is {}", c.get_radius());

    // 静态方法
    struct T(i32);
    impl T {
        fn func(this: &Self) -> () {
            println!("inner value is {}", this.0);
        }
    }
    let x = T(23);
    T::func(&x);

    // fully qualified syntax
    trait Cook {
        fn start(&self);
    }
    trait Wash {
        fn start(&self);
    }
    struct Chef;
    impl Cook for Chef {
        fn start(&self) { println!("Cook::start"); }
    }
    impl Wash for Chef {
        fn start(&self) { println!("Wash::start"); }
    }
   
    let chef = Chef {};
    Cook::start(&chef);

    // trait约束
    use std::fmt::Debug;
    fn my_print<T : Debug>(x: T) {
        println!("The value is {:? }.", x);
    }
    my_print("China");
    my_print(41_i32);
    my_print(true);
    my_print(['a', 'b', 'c']);

    // trait dynamic dispatch
    trait FooBar {
        fn method(&self) -> String;
    }
    impl FooBar for u8 {
        fn method(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl FooBar for String {
        fn method(&self) -> String {
            format!("string: {}", *self)
        }
    }
    let x = 23u8;
    fn coerce_foobar(foo: &dyn FooBar) {
        println!("{}", foo.method());
    }
    coerce_foobar(&x);
}