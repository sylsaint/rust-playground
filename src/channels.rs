use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn channels_run() {
    // mpsc = multiple producer single consumer
    let (tx, rx) = mpsc::channel();
    let vals = vec![String::from("Hi"), String::from("from"), String::from("the"), String::from("thread")];
    let tx1 = tx.clone();
    thread::spawn(move || {
        for msg in vals {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    let more_vals = vec![String::from("more"), String::from("messages"), String::from("for"), String::from("you")];
    thread::spawn(move || {
        for msg in more_vals {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for recv in rx {
        println!("Got: {}", recv);
    }
}