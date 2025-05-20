use std::collections::HashMap;

pub fn hashmap_run() {
    let s1 = String::from("hello");
    let s2 = String::from("hello");
    println!("s1 == s2 is {}", s1 == s2);
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(s1.clone(), 50);
    println!("get Blue key: {}", scores.get("Blue").unwrap());
    // s1 ownership has been moved, you can not use it anymore!
    println!("s1 valid: {}", s1);

}