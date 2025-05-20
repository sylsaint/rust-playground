use std::borrow::Cow;
fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        return Cow::Owned(buf);
    }
    return Cow::Borrowed(input);
}
pub fn cow_run() {
    let s1 = "no_spaces_in_string";
    let mut result1 = remove_spaces(s1);
    println!("s1 addr: {:p}, result1 addr: {:p}", s1, result1.as_ref());
    let mut ss = result1.to_mut();
    println!("ss addr: {:p}", ss);
    println!("s1 addr: {:p}, result1 addr: {:p}", s1, result1.as_ref());
    let ss1 = result1.as_ref();
    println!("deref result1 addr: {:p}", &ss1);
    let s2 = "spaces in string";
    let result2 = remove_spaces(s2);
    println!("s2 addr: {:p}, result2 addr: {:p}", s2, result2.as_ref());
    println! ("{}\n{}", result1, result2);
    println!("result1 addr: {:p}, result2 addr: {:p}", result1.as_ref(), result2.as_ref());
}