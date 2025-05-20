pub fn iter_run() {
    let iter = vec![1, 2, 3];
    let iter1 = iter.iter();
    for v in iter1 {
        println!("element {}", v);
    }
}