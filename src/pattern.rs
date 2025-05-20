#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn pattern_run() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'a'..='z' => println!("late ASCII letter"),

        _ => println!("something else"),
    }
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("origin p is {:?}", p.x);

    let origin = (1, 2, 3, 4, 5); 

    match origin {
        ( x, rest @ _, ..) => println!("second is {}", rest),
    }
}