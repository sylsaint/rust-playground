use std::ops::Deref;
use std::rc::Rc;

pub fn chapter16_run() {
    fn type_of(_: ()) {}
    let s = Rc::new(Rc::new(String::from("hello")));
    let s1 = s.clone(); //(1)
                        //type_of(s1);
    let ps1 = (*s).clone(); //(2)
                            //type_of(ps1);
    let pps1 = (**s).clone(); //(3)
                              //type_of(pps1);

    // 引用计数复制后，执行内存地址一样
    struct SharedValue {
        value: i32,
    }
    let shared_value: Rc<SharedValue> = Rc::new(SharedValue { value: 42 });
    let owner1 = shared_value.clone();
    let owner2 = shared_value.clone();
    println!("value : {} {}", owner1.value, owner2.value);
    println!("address : {:p} {:p}", &owner1.value, &owner2.value);

    // cow
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
    let s1 = "no_spaces_in_string";
    let result1 = remove_spaces(s1);
    let rs1 = result1.as_ref();
    let s2 = "spaces in string";
    let result2 = remove_spaces(s2);
    println!("{}\n{}", result1, result2);
}
