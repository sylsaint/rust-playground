pub fn intrin_run() {
    let x = vec![1, 2, 3, 4, 5];

    println!("vec capacity is {}", x.capacity());

    unsafe {
        let t: (usize, usize, usize) = std::mem::transmute_copy(&x);
        println!("{} {} {}", t.0, t.1, t.2);
    }
    let p = x.as_ptr() as *const usize;
    let p1 = &x as *const _ as *const usize;
    unsafe {
        println!("raw is {}", *x.as_ptr().add(1));
        println!("point is {}", *p);
        println!("p1 is {}", *p1.add(1));
    }
}
