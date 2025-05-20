use std::cell::RefCell;

pub fn chapter17_run() {
    use std::rc::Rc;
    struct Node {
        next: Option<Rc<RefCell<Node>>>,
    }
    impl Drop for Node {
        fn drop(&mut self) {
            println!("drop");
        }
    }
    fn alloc_objects() {
        let node1 = Rc::new(RefCell::new(Node { next: None }));
        let node2 = Rc::new(RefCell::new(Node { next: None }));
        let node3 = Rc::new(RefCell::new(Node { next: None }));
        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node1.clone());
    }
    alloc_objects();
    println!("program finished");
}
