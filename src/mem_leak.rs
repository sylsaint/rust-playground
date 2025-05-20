use std::{rc::Rc, cell::RefCell};

struct Node {
    val: isize,
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("node {} has been dropped", self.val);
    }
}

pub fn mem_leak_run() {
    let node1 = Rc::new(RefCell::new(Node { val: 10, next: None }));
    let node2 = Rc::new(RefCell::new(Node { val: 20, next: None }));
    node1.borrow_mut().next = Some(Rc::clone(&node2));
    node2.borrow_mut().next = Some(Rc::clone(&node1));
}