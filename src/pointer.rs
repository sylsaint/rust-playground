use std::rc::Rc;

/*
enum List {
    Cons(i32, List),
    Nil,
}
*/

#[derive(Debug)]
enum ListSmart {
    Cons(i32, Box<ListSmart>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

pub fn pointer_run() {
    use crate::pointer::ListSmart::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let mut p = list;
    loop {
        match p {
            Nil => {
                println!("finished");
                break;
            }
            Cons(num, next) => {
                println!("element is {}", num);
                p = *next;
            }
        }
    }
    println!("list is {:?}", p);
    let x = 5;
    let y = &x;

    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let z = MyBox::new(x);
    let name = Box::new(String::from("Rust"));
    assert_eq!(5, *z);
    println!("x is {}", *z);
    fn hello(name: &str) {
        assert_eq!("Rust", name);
        println!("name is {}", name);
    }
    hello(&name);
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
    let a = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    println!("stront ref count: {}", Rc::strong_count(&a));
    let b = ListRc::Cons(3, Rc::clone(&a));
    println!("stront ref count: {}", Rc::strong_count(&a));
    let c = ListRc::Cons(4, Rc::clone(&a));
    println!("stront ref count: {}", Rc::strong_count(&a));
    drop(b);
    println!("stront ref count: {}", Rc::strong_count(&a));
    drop(c);
    println!("stront ref count: {}", Rc::strong_count(&a));
}
