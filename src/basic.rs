pub fn variable_run() {
    let mut x1 = 10;
    let x2 = x1;
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);

    // shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner scope x = {}", x);
    }
    println!("outer scope x = {}", x);

    /*
     * DataTypes
     */
    // overflow test
    let mut i1: u8 = 254;
    println!("u8 overflow 254 + 10 = {}", i1);

    // tuple
    let tup = (1, 2, "hehe");
    println!("tup.2 is {}", tup.2);

    // array
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let pivot = 1;
    let r = arr1.binary_search(&pivot).expect("not found");
    println!("arr1 binary search pos {}", r);

    // functions
    fn echo(x: i32) -> i32 {
        x
    }
    fn no_return() {}
    println!("echo(8) is {}", echo(8));
    println!("no_return() is {:?}", no_return());

    // control flow
    let number = 6;
    if {
        number % 2 == 0;
        false
    } {
        println!("{} is a even number", number);
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter after loop assignment is {}", result);

#[derive(Clone)]
    struct F {
        name: i32,
        next: Box<i32>,
    }
    let mut n1 = Box::new(31_i32);
    let mut f1 = F { name: 32, next: n1 };
    let mut f2 = f1.clone(); 
    println!("f1 next addr {:p}", f1.next);
    println!("f2 next addr {:p}", f2.next);
    *f2.next = 40;
    println!("f1 next {}", *f1.next);
    println!("f2 next {}", *f2.next);
    *f1.next = 50;
    println!("f1 next {}", *f1.next);
    println!("f2 next {}", *f2.next);

    println!("size of bool {:?}", std::mem::size_of::<F>());
    println!("size of char {:?}", std::mem::size_of::<char>());
    println!("size of i32 {:?}", std::mem::size_of::<i32>());
    println!("size of i64 {:?}", std::mem::size_of::<i64>());
}
