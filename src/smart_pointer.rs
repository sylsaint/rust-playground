use std::rc::Rc;
struct SharedValue {
    value : i32
}
pub fn smart_pointer_run() {
    let shared_value : Rc<SharedValue> = Rc::new(SharedValue { value : 42 });
    let owner1 = shared_value.clone();
    let owner2 = shared_value.clone();
    println! ("value : {} {}", owner1.value, owner2.value);
    println! ("address : {:p} {:p}", &owner1.value, &owner2.value);
}