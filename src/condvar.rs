use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

pub fn condvar_run() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        thread::sleep(Duration::from_secs(1));
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("child thread {}", *started);
    });

    // wait for the thread to start up
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    println!("before wait {}", *started);
    if !*started {
        println!("need wait");
        started = cvar.wait(started).unwrap();
    }
    println!("after wait {}", *started);
}