pub fn chapter13_run() {
    let mut x = String::from("hello");
    let mx = &mut x; 
    println!("str len = {}, pointer add is {:p}", mx.len(), &mx);
    let mut i = 0;
    while i < 10000 {
        mx.push_str("heihei,");
        i += 1;  
    }
    println!("str len = {}, pointer add is {:p}", mx.len(), &mx);

    let s1 = String::from("hello world");
    let mut s2 = String::from("hello world");
    let ps1 = &s1;
    let psp1 = s1.as_ptr();
    let ps2 = &s2;
    let psp2 = s2.as_ptr();

    println!("{:p}, {:p}", ps2, psp2); // 0x24bf3dc7e30
    let mut i = 0;
    while i < 100 {
        s2.push_str(", hello");
        i += 1;
    }

    println!("{:p}, {:p}", ps1, psp1); // 0x24bf3dc7e30
    println!("s2 len {}, {:p}", s2.len(), s2.as_ptr()); // 0x24bf3dc8190

}