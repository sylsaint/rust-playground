use std::mem;
use std::ptr;

pub fn chapter19_run() {
    let mut y: u32 = 1;
    let x = 1_i32;
    println!("y addr: {:p}", &y);
    println!("x addr: {:p}", &x);

    let raw_mut = &mut y as *mut u32 as *mut i32 as *mut i64; // 这是安全的

    unsafe {
        *raw_mut = -1; // 这是不安全的，必须在 unsafe 块中才能通过编译
    }

    println!("{:X} {:X}", x, y);

    fn swap<T>(x: &mut T, y: &mut T) {
        unsafe {
            let mut t = mem::MaybeUninit::<T>::uninit();
            let pt = t.as_mut_ptr();
            ptr::copy_nonoverlapping(&*x, pt, 1);
            ptr::copy_nonoverlapping(&*y, x, 1);
            ptr::copy_nonoverlapping(pt, y, 1);
            mem::forget(t);
        }
    }
    let mut x = 10;
    let mut y = 20;
    println!("x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("x = {}, y = {}", x, y);
}
