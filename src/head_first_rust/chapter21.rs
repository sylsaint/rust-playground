use std::cmp::PartialOrd;

fn compare_option<T>(first: Option<T>, second: Option<T>) -> bool {
    match (first, second) {
        (Some(..), Some(..)) => true,
        (None, None) => true,
        _ => false,
    }
}

fn max<T>(a: T, b: T) -> T where T: PartialOrd {
    if a < b { b } else { a }
}

trait Iter {
    type Item;
    fn max(&self) -> Option<Self::Item> 
        where Self: Sized, Self::Item: PartialOrd;
}

use std::iter::Iterator;
use std::fmt::Debug;

fn use_iter<ITEM, ITER>(mut iter: ITER)
    where ITER: Iterator<Item=ITEM>,
            ITEM: Debug
{
    while let Some(i) = iter.next() {
        println!("{:? }", i);
    }
}

trait Graph {
    type Edge;
    type Vertex;
    fn has_edge(&self, v1: &Self::Vertex, v2: &Self::Vertex) -> bool;
}

pub fn chapter21_run() {
    let i1: i16 = (32u8).into();
    let i2 = i32::from(i1);
    println!("now i2 is {}", i2);
    let v: Vec<i32> = vec! [1,2,3,4,5];
    use_iter(v.iter());
    println!("vec is {:?}", v);
}