use std::collections::VecDeque;
use std::collections::BTreeMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct BPerson {
    first_name: String,
    last_name: String,
}

impl BPerson {
    fn new(first: &str, last: &str) -> Self {
        BPerson {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}

struct NumberGenerator {
    v: i32
}

impl Iterator for NumberGenerator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v < 100 {
            self.v += 1;
            return Some(self.v);
        }
        return None; 
    }
}

pub fn chapter24_run() {
    // 常见的几种构造Vec的方式
    // 1. new() 方法与 default() 方法一样，构造一个空的Vec
    let v1 = Vec::<i32>::new();
    // 2. with_capacity() 方法可以预先分配一个较大空间，避免插入数据的时候动态扩容
    let v2: Vec<String> = Vec::with_capacity(1000);
    // 3. 利用宏来初始化，语法跟数组初始化类似
    let v3 = vec![1, 2, 3];

    // 插入数据
    let mut v4 = Vec::new();
    // 多种插入数据的方式
    v4.push(1);
    v4.extend_from_slice(&[10, 20, 30, 40, 50]);
    v4.insert(2, 100);
    println!("capacity: {} length: {}", v4.capacity(), v4.len());

    // 访问数据
    // 调用 IndexMut 运算符，可以写入数据
    v4[5] = 5;
    let i = v4[5];
    println!("{}", i);
    // Index 运算符直接访问，如果越界则会造成 panic，而 get 方法不会，因为它返回一个 Option<T>
    if let Some(i) = v4.get(6) {
        println!("{}", i);
    }
    // Index 运算符支持使用各种 Range 作为索引
    let slice = &v4[4..];
    println!("{:? }", slice);

    // 双端队列

    let mut queue = VecDeque::with_capacity(64);
    // 向尾部按顺序插入一堆数据
    for i in 1..10 {
        queue.push_back(i);
    }
    // 从头部按顺序一个个取出来
    while let Some(i) = queue.pop_front() {
        println!("{}", i);
    }

    // hashmap
    use std::collections::HashMap;

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        fn new(first: &str, last: &str) -> Self {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }
    }
    let mut book = HashMap::new();
    book.insert(Person::new("John", "Smith"), "521-8976");
    book.insert(Person::new("Sandra", "Dee"), "521-9655");
    book.insert(Person::new("Ted", "Baker"), "418-4165");

    let p = Person::new("John", "Smith");

    // 查找键对应的值
    if let Some(phone) = book.get(&p) {
        println!("Phone number found: {}", phone);
    }

    // 删除
    book.remove(&p);

    // 查询是否存在
    println!("Find key: {}", book.contains_key(&p));

    // btree
    let mut book1 = BTreeMap::new();
    book1.insert(BPerson::new("John", "Smith"), "521-8976");
    book1.insert(BPerson::new("Sandra", "Dee"), "521-9655");
    book1.insert(BPerson::new("Ted", "Baker"), "418-4165");

    let p: BPerson = BPerson::new("John", "Smith");

    // 查找键对应的值
    if let Some(phone) = book1.get(&p) {
        println!("Phone number found: {}", phone);
    }

    // 删除
    book1.remove(&p);

    // 查询是否存在
    println!("Find key for {:?}: {}", &p, book1.contains_key(&p));

    let mut map = BTreeMap::new();
    map.insert(3, "a");
    map.insert(5, "b");
    map.insert(8, "c");
    for (k, v) in map.range(2..6) {
        println!("{} : {}", k, v);
    }
    let mut it = NumberGenerator { v: 0 };
    while let Some(v) = it.next() {
        println!("v is {}", v);
    }

}
