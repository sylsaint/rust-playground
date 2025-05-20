extern "C" {
    fn abs(i: i32) -> i32;
}

static mut COUNTER: i32 = 10; 

pub fn unsafety_run() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let addr = 0x309a5f9ccusize;
    let p = r2 as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r1 addr is: {:?}", r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        println!("p point {}", *p);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    unsafe {
        println!("abs of value -3 is {}", abs(-3));
    }
    fn add_to_count(inc: i32) {
        unsafe {
            COUNTER += inc;
        }
    }
    
    add_to_count(3);
    unsafe {
        println!("now COUNTER is {}", COUNTER);
    }
    
}