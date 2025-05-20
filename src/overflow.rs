use std::panic;
use std::thread;

pub fn overflow_run() {
    let t = thread::Builder::new()
        .stack_size(1024 * 1024 * 10)
        .name("growing".to_string())
        .spawn(|| {
            let r = panic::catch_unwind(|| {
                let y: [u64; 10000000] = [1; 10000000];
            });
            match r {
                Ok(_) => {
                    println!(" run without errors ")
                }
                Err(e) => {
                    println!(" catch errors {:?}", e)
                }
            }
        });
    match t {
        Ok(r) => {
            r.join().ok();
        }
        Err(e) => {
            println!("error occured {:?}", e);
        }
    }
}
