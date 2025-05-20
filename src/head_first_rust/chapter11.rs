use std::mem::drop;

struct T {
    value: i32,
}

pub fn chapter11_run() {
    let b = Box::new(T { value: 10 });
    println!("b.value is {}", b.value);
    let c = b;
    // b is moved, not available
    // println!("b is no available, {}", b.value);

    // copy 语义
    let v1 = 10i32;
    let v2 = true;
    let v3 = &v2;
    let v4 = v1;
    let v5 = v2;
    let v6 = v3;
    println!("after re-assign with Copy trait, v1 = {}, v2 = {}, v3 = {}", v1, v2, v3);
    let v7 = [1,2,3];
    let v8 = v7;
    println!("after re-assign with Copy trait, v7 = {:?}", v7);
    let v9 = (23i32, true);
    let v10 = v9; 
    println!("after re-assign with Copy trait, v9 = {:?}", v9);

    // 主动析构
    drop(v1);
    println!("now v1 is still available, {}", v1);
}
