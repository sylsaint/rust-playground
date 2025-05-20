trait T1 {
    fn get_name(&self) -> &String;
}

trait T2: T1 {
    fn get_age(&self) -> usize;
}

trait T3: T2 {
    fn get_weight(&self) -> usize;
}

struct ST1 {
    name: String,
}

impl T1 for ST1 {
    fn get_name(&self) -> &String {
        &self.name
    }
}

struct ST2 {
    name: String,
    age: usize,
}

impl T1 for ST2 {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl T2 for ST2 {
    fn get_age(&self) -> usize {
        self.age
    }
}

struct ST3 {
    name: String,
    age: usize,
    weight: usize,
}

impl T1 for ST3 {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl T2 for ST3 {
    fn get_age(&self) -> usize {
        self.age
    }
}

impl T3 for ST3 {
    fn get_weight(&self) -> usize {
        self.weight
    }
}

fn echo1(x: & dyn T1) -> () {
    println!("name is {}", x.get_name());
}

fn echo2(x: & dyn T2) -> () {
    println!("age is {}", x.get_age());
}

fn echo3(x: & dyn T3) -> () {
    println!("weight is {}", x.get_weight());
}

fn high<F>(f1: &F) where F: Fn(& dyn T2) -> () {
    f1(&ST2{name: String::from("hehe"), age: 30usize});
}

pub fn variant_run() {
    high(&echo2)
}