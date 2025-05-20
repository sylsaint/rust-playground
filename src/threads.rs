use std::thread;
use std::time::Duration;

pub fn threads_run() {
    // 如果不使用join方法，那么只会输出5次，因为主线程退出之后，子线程也会随之退出
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        let local = v;
        println!("Here's a vector: {:?}", local);
    });

    handle.join().unwrap();
}