struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct UserRef<'a, 'b> {
    active: bool,
    username: &'a str,
    email: &'b str,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u64) {
        self.width = width;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn build(width: u64, height: u64) -> Self {
        Self {
            width,
            height
        }
    }
}

pub fn struct_run() {
    let user1 = User {
        active: true,
        username: String::from("ylsun"),
        email: String::from("limerary@gmail.com"),
        sign_in_count: 999,
    };

    let user2 = User {
        username: String::from("hehe"),
        ..user1
    };
    println!("user2 name {}", user2.username);

    // struct with ref
    let username = String::from("hehe");
    let email = String::from("haha");
    let user_ref = UserRef {
        active: true,
        username: &username,
        email: &email,
        sign_in_count: 10,
    };
    println!("user_ref {:?}", user_ref);

    let scale = 10;
    let mut rect1 = Rectangle {
        width: 20,
        height: dbg!(30 * scale),
    };
    let rect2 = Rectangle {
        width: 30,
        height: dbg!(10 * scale),
    };
    dbg!(&rect1);
    println!("area of the rectangle is {}", rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    rect1.set_width(80);
    println!("set width of rect1 to {}", 80);
    println!("area of the rectangle is {}", rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    let rect3 = Rectangle::build(3, 4);
    println!("rect3 built {:?}", rect3);
}