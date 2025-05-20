use unicode_segmentation::UnicodeSegmentation; // 1.5.0

pub fn collections_run() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("vector is {:?}", v);

    // modify element of vector
    for i in &mut v {
        *i += 50;
    }
    println!("vector is {:?}", v);

    // use enum to contain different types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("vector is {:?}", row);

    // string underlying representation
    for c in "Зд".chars() {
        println!("{c}");
    }    
    for c in "Зд".bytes() {
        println!("{c}");
    }
    // bytes scalar grapheme cluster
    let hindi = "नमस्ते";
    println!("hindi bytes is {:?}", hindi.bytes());
    println!("hindi char is {:?}", hindi.chars());
    for g in hindi.graphemes(true) {
        println!("- {}", g);
    }
}