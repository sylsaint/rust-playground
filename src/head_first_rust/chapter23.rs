use std::{mem, fmt::Debug};

trait Bird {
    fn fly(&self);
}

struct Duck;
struct Swan;

impl Bird for Duck {
    fn fly(&self) {
        println!("I am a Duck");
    }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("I am a Swan");
    }
}

fn test<T: Bird>(bird: T) {
    bird.fly();
}

trait Animal<'a> {
    fn get_name(&'a self) -> &'a str;
}
struct Flamingo<'a> {
    name: &'a str 
}
struct Crocdile<'a> {
    name: &'a str
}

impl <'a> Animal<'a> for Flamingo<'a> {
    fn get_name(&'a self) -> &'a str {
        &self.name
    }
}
impl <'a> Animal<'a> for Crocdile<'a> {
    fn get_name(&'a self) -> &'a str {
        &self.name
    }
}

fn show_animal_name<'a, T: Animal<'a>>(animal: &'a T) {
    println!("animal name is: {}", animal.get_name());
}

// 参数是 trait object 类型，p 是一个胖指针
fn print_traitobject(p: &dyn Bird) {
    // 使用transmute执行强制类型转换，把变量p的内部数据取出来
    let (data, vtable): (usize, *const usize) = unsafe { mem::transmute(p) };
    println!("TraitObject    [data:{}, vtable:{:p}]", data, vtable);
    unsafe {
        // 打印出指针 v 指向的内存区间的值
        println!(
            "data in vtable [{}, {}, {}, {}]",
            *vtable,
            *vtable.offset(1),
            *vtable.offset(2),
            *vtable.offset(3)
        );
    }
}

// 约束Self:Sized
trait Phone {
    fn show_model(&self);
    fn show_model_with_constraint(&self) where Self:Sized;
}

struct iPhone<'a> {
    model: &'a str
}

impl <'a> Phone for iPhone<'a> {
    fn show_model(&self) {
        println!("model: {}", self.model);    
    }

    fn show_model_with_constraint(&self) where Self:Sized {
        println!("model: {}", self.model);    
    }
}

trait SomeTrait {
    fn generic_fn<T: Debug>(&self, t: T);
}

struct Bang;

impl SomeTrait for Bang {
    fn generic_fn<T: Debug>(&self, t: T) {
        println!("{:?}", t);
    }
}

fn multiply(m: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * m)
}

use std::iter;
use std::vec::IntoIter;

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn do_impl_return() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");
}

pub fn chapter23_run() {
    let duck: Duck = Duck {};
    test(duck);
    let duck = Duck;
    let p_duck = &duck;
    let p_bird = p_duck as &dyn Bird;
    println!(
        "Size of p_duck {}, Size of p_bird {}",
        mem::size_of_val(&p_duck),
        mem::size_of_val(&p_bird)
    );

    let duck_fly: usize = Duck::fly as usize;
    let swan_fly: usize = Swan::fly as usize;
    println!("Duck::fly {}", duck_fly);
    println!("Swan::fly {}", swan_fly);

    print_traitobject(p_bird);
    let swan = Swan;
    print_traitobject(&swan as &dyn Bird);

    let flamingo = Flamingo { name: &String::from("flamingo")};
    show_animal_name(&flamingo);
    let ip15 = iPhone { model: &"iPhone 15 Promax" };
    ip15.show_model();
    ip15.show_model_with_constraint();
    // 无法进行强制类型转换
    let dyn_ip15 = &ip15 as &dyn Phone;
    dyn_ip15.show_model();
    // dyn_ip15.show_model_with_constraint();
    let bang = Bang;
    bang.generic_fn(20);
    bang.generic_fn(String::from("hehe"));

    // 指向闭包的指针
    let p = multiply(10);
    println!("10 x 20 = {}", p(20));
    let pu8 = Box::new(8);
    println!("pu8 is {}", pu8);

    //
    do_impl_return();
}
