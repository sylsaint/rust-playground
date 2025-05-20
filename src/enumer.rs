#[derive(PartialEq)]
enum IpAddrNew {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
   fn call(&self) {
    match self {
        Message::Quit => println!("this is quit"),
        Message::Write(xs) => println!("this is write, {}", xs),
        Message::Move { x: _, y: _} => println!("this is move"),
        Message::ChangeColor(_, _, _) => println!("this is change color"),
    }
    dbg!(&self);
   } 
}

pub fn enumer_run() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);
    // compare ipv4
    let ip1 = IpAddrNew::V4(127, 0, 0, 1);
    let ip2 = IpAddrNew::V4(127, 0, 0, 1);
    println!("ip2 == ip2 is {}", ip1 == ip2);
    let m = Message::Write(String::from("hello"));
    m.call();

    // enum option
    let opt_a = Some(12);
    println!("opt a is {}", opt_a.unwrap());
}