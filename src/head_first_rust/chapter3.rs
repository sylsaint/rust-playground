pub fn chapter3_run() {
    let num1: u8 = 0b_1010_1010;
    let num2: u8 = 0b_1111_0000;
    println!("{:08b}", !num1);
    println!("{:08b}", num1 & num2);
    println!("{:08b}", num1 | num2);
    println!("{:08b}", num1 ^ num2);
    println!("{:08b}", num2 << 4);
    println!("{:08b}", num2 >> 4);

    // 赋值中的copy和move
    let num3 = num1;
    println!("num1 is {}", num1);

    #[derive(Clone, Copy)]
    struct Foo {
        bar: i32
    }

    // 实现Copy trait
    let f1 = Foo { bar: 20 };
    let f2 = f1;
    println!("f1 is only available when Foo has Clone, Copy trait with it, {}", f1.bar);
    
}
