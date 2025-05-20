enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: i32) -> i32 {
    match x {
        25 => 24,
        i => i + 1,
    }
}

pub fn match_run() {
    let coin = Coin::Quarter;
    let cnt = value_in_cents(coin);
    println!("a quarter coin is {}/100 dollar", cnt);
    println!("plus one {}", plus_one(25));
    println!("plus one {}", plus_one(26));
    let config_max = Some(3u8);
    if let Some(x) = config_max {
        println!("the max value is {}", x);
    }
    // 等价于
    match config_max {
        Some(max) => println!("the max value is {}", max),
        _ => (),
    }
}