use std::sync::{Mutex, Arc};
use std::thread;

const COUNT: u32 = 1000000;

pub fn mutexs_run() {
    let global = Arc::new(Mutex::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move|| {
        for _ in 0..COUNT {
            let mut value = clone1.lock().unwrap();
            *value += 2;
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move|| {
        for _ in 0..COUNT {
            let mut value = clone2.lock().unwrap();
            *value -= 1;
        }
    });
    let clone3 = global.clone();
    let thread3 = thread::spawn(move|| {
        for _ in 0..100usize {
            let value = clone3.as_ref();
            println!("I see value {:?}", value);
        }
    });

    thread1.join().ok();
    thread2.join().ok();
    thread3.join().ok();
    println!("final value: {:? }", global);
}