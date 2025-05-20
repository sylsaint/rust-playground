use std::cell::{Cell, RefCell};

struct CellV1<T> {
    value: T
}

impl<T> CellV1<T> {
    fn new(v: T) -> CellV1<T> {
        CellV1 { value: v }
    }
    fn set<'a, 'b>(&'a self, v: &'b T) 
    where T: Copy,
    'b : 'a 
    {
        unsafe {
            let p = &self.value as *const T as *mut T;
            // *p = *v;
        }
    }
    fn get(&self) -> T where T: Copy {
        self.value
    }
}

pub fn chapter15_run() {
    let x = 30;
    let c = Cell::new(x);
    println!("x is {}", x);
    let rc1 = &c;
    let rc2 = &c;
    println!("rc1 is {:?}", rc1);
    println!("rc2 is {:?}", rc2);
    rc1.set(20);
    println!("rc1 is {:?}", rc1);
    println!("rc2 is {:?}", rc2);
    rc2.set(30);
    println!("rc1 is {:?}", rc1);
    println!("rc2 is {:?}", rc2);

    // refcell
    let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec! [1, 2, 3]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;
    println!("shared1 addr {:p}", shared1);
    println!("shared2 addr {:p}", shared2);
    println!("shared2 2 addr {:p}", *shared2);
    shared1.borrow_mut().push(4);
    println! ("{:? }", shared_vec.borrow());
    shared2.borrow_mut().push(5);
    println! ("{:? }", shared_vec.borrow());

    let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec! [1, 2, 3]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;
    let p1 = shared1.borrow();
    let p2 = &p1[0];
    // shared2.borrow_mut().push(4);
    // println! ("{}", p2);

    // evil
    struct Table<'arg> {
        cell: CellV1<&'arg isize>
    }
    fn evil<'long, 'short>(t: &Table<'long>, s: &'short isize)
        where 'short : 'long
    {
        // The following assignment is not legal, but it escapes from lifetime checking
        let u: &Table<'long> = t;
        u.cell.set(&s);
    }
    fn innocent<'long>(t: &Table<'long>) {
        let foo: isize = 1;
        evil(t, &foo);
    }
    let local = 100;
    let table = Table { cell: CellV1::new(&local) };
    innocent(&table);
    // reads `foo`, which has been destroyed
    let p = table.cell.get();
    println! ("{}", p);
}