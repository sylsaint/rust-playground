use std::sync::{Arc, Barrier};
use std::thread;

pub fn barrier_run() {
    let barrier = Arc::new(Barrier::new(10));
    let mut handlers = vec! [];
    for _ in 0..10 {
        let c = barrier.clone();
        // The same messages will be printed together.
        // You will NOT see any interleaving.
        let t = thread::spawn(move|| {
            println!("before wait");
            let r = c.wait();
            println!("barrier result {:?}", r);
            println!("after wait");
        });
        handlers.push(t);
    }

    for h in handlers {
        h.join().ok();
    }
}
