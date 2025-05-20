pub fn chapter18_run() {
    let nk: Option<i32> = None;
    let inner = nk.unwrap_or_default();
    println!("inner is {}", inner);
    use std::panic;

    panic::catch_unwind(|| {
        let x: Option<i32> = None;
        x.unwrap();
        println!("interrupted.");
    })
    .ok();

    println!("continue to execute.");
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    const COUNT: u32 = 1000000;

    let global = Arc::new(Mutex::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            match clone1.lock() {
                Ok(mut value) => *value += 1,
                Err(poisoned) => {
                    let mut value = poisoned.into_inner();
                    *value += 1;
                }
            }
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::Builder::new().name("bad".to_string()).spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone2.lock().unwrap();
            *value -= 1;
            println!("current value is {}", *value);
            if *value < 100000 {
                println!("make a panic");
                panic!("");
            }
        }
    });

    thread1.join().ok();
    thread2.unwrap().join().ok();
    println!("final value: {:? }", global);
}
