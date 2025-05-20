fn show_arr(xs: [i32; 5]) {
    println!("arr is {:?}", xs);
}

pub fn chapter6_run() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    show_arr(xs);
    println!("first elem = {}", xs[0]);
    let xs1 = xs;
    println!("first elem = {}", xs1[0]);
    println!("first elem = {}", xs[0]);

    #[derive(Debug, Copy, Clone)]
    struct Foo {
        value: i32,
    }

    let fs: [Foo; 2] = [Foo { value: 3 }, Foo { value: 2 }];
    let fs1 = fs;
    println!("first elem = {:?}", fs[0]);

    // 多维数组
    let mut rss: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("rss_1_2 is {}", rss[1][2]);

    // 数组切片
    let slice: &mut [[i32; 3]; 3] = &mut rss;
    slice[1][2] = 10;
    println!("rss_1_2 is {}", rss[1][2]);

    // fat pointer
    fn raw_slice(arr: &[i32]) {
        unsafe {
            let (val1, val2): (usize, usize) = std::mem::transmute(arr);
            println!("Value in raw pointer:");
            println!("value1: {:x}", val1);
            println!("value2: {:x}", val2);
        }
    }
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let address: &[i32; 5] = &arr;
    println!("Address of arr: {:p}", address);
    raw_slice(address as &[i32]);
    struct High<'a> {
        value: isize,
        head: &'a [i32],
        tail: &'a [i32],
    }
    fn show_name(x: &High) {
        println!("{} {}", x.head[0], x.tail[0]);
    }

    let h1 = High {
        value: 10,
        head: &[1, 2, 3],
        tail: &[2, 3, 4],
    };
    show_name(&h1);
    // 字符串富指针
    println!("Size of pointer : {}", std::mem::size_of::<*const ()>());
    println!("Size of &str    : {}", std::mem::size_of::<&str>());
    // String to str
    fn capitalize(s: &mut str) {
        s.make_ascii_uppercase();
    }
    let mut ss = String::from("brush my teeth");
    println!("ss is {}", ss);
    capitalize(&mut ss);
    println!("ss is {}", ss);

    // 
}
