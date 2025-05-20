use std::mem::{size_of_val, size_of};

struct Zero {}
#[derive(Debug)]
struct NonZero {
    value: isize
}

struct T { dropped: bool }

impl T {
    fn new() -> Self {
        T { dropped: false }
    }
}

impl Drop for T {
    fn drop(&mut self) {
        self.dropped = true;
    }
}

struct R<'a> {
    inner: Option<&'a T>
}

impl<'a> R<'a> {
    fn new() -> Self {
        R { inner: None }
    }
    fn set_ref<'b :'a>(&mut self, ptr: &'b T) {
        self.inner = Some(ptr);
    }
}

impl<'a> Drop for R<'a> {
    fn drop(&mut self) {
        if let Some(ref inner) = self.inner {
            println! ("droppen R when T is {}", inner.dropped);
        }
    }
}

pub fn chapter20_run() {
    println!("size of type Zero: {}", size_of::<Zero>());
    let void_vec: Vec<Zero> = Vec::new();
    println!("size of void vec is {}", size_of_val(&void_vec));
    println!("vec capacity is {}", void_vec.capacity());
    println!("vec ptr is {:p}", void_vec.as_ptr());
    let vec_20: Vec<Zero> = Vec::with_capacity(20);
    println!("size of vec_20 is {}", size_of_val(&vec_20));
    println!("vec_20 capacity is {}", vec_20.capacity());
    println!("vec_20 ptr is {:p}", vec_20.as_ptr());
    let mut vec_20_nonzero: Vec<NonZero> = Vec::with_capacity(20);
    println!("size of vec_20_nonzero is {}", size_of_val(&vec_20_nonzero));
    println!("vec_20_nonzero capacity is {}", vec_20_nonzero.capacity());
    println!("vec_20_nonzero ptr is {:p}", vec_20_nonzero.as_ptr());

    // drop check
    {
        let (a, mut b) : (T, R) = (T::new(), R::new());
        b.set_ref(&a);
    }
    {
        let (mut a, b) : (R, T) = (R::new(), T::new());
        // a.set_ref(&b);
    }

    // panic safety
    let m = &vec_20_nonzero.get(11);
    // vec_20_nonzero.truncate(10);
    println!("m is {:?}", m);
}