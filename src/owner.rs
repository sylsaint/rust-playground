fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
fn change(s: &mut String) {
    s.push_str(", world");
    println!("change: {}", s);
}
fn dangle() {
    let s = String::from("hello");
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
pub fn owner_run() {
    let s = "abcdefg";
    let s1 = String::from("hellow");
    // s1.push_str(", world");
    // will cause error, because s1 is invalid
    // let s2 = s1;
    println!("{}", s1);
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    let ss = String::from("hello");
    let mut ss_copy = String::from("hello"); take_ownership(ss);
    println!("{}", ss_copy);
    // reference, following function causes "cannot borrow as mutable"
    // change(ss_copy);
    // change(&mut ss_copy);
    let r1 = &mut ss_copy;
    {
        println!("borrow r1: {}", r1);
        let r2 = &mut ss_copy;
    }
    // let r3 = &mut ss_copy;

    // first immutable then mutable
    let im = String::from("heihei");
    let rim1 = &im;
    let rim2 = &im;
    // cause error
    // let rim3 = &mut im;
    println!("rim1 = {}, rim2 = {}", rim1, rim2);

    // dangle error
    // let ref_to_nothing = dangle();

    // slice
    let mut hw = String::from("Hello World!");
    let word = first_word(&hw);
    // hw.clear();
    println!("first word is {}", word);
    

}