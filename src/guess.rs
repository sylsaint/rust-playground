pub mod guessing {
    use rand::Rng;
    use std::{cmp::Ordering, io};

    pub fn guess() {
        println!("Guess the number!");
        println!("Please input your guess.");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        loop {
            let mut gs = String::new();
            io::stdin().read_line(&mut gs).expect("Failed to read line");
            println!("You guessed: {gs}");
            let guess: u32 = match gs.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                }
            };
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("you win!");
                    break;
                }
            }
        }
    }
}
