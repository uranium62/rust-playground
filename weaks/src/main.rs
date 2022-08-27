use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct SnakePart {
    id: i32,
    name: String,
    next: RefCell<Weak<SnakePart>>,
    prev: RefCell<Option<Rc<SnakePart>>>,
}


fn main() {
    let tail = Rc::new(SnakePart {
        id: 1,
        name: "tail #1".to_string(),
        next: RefCell::new(Weak::new()),
        prev: RefCell::new(None)
    });
    
    println!("tail.next = {:?}", tail.next.borrow().upgrade());

    let head = Rc::new(SnakePart {
        id: 0,
        name: "head #1".to_string(),
        next: RefCell::new(Weak::new()),
        prev: RefCell::new(Some(tail.clone()))
    });
    
    *(tail.next.borrow_mut()) = Rc::downgrade(&head);

    println!("tail.next = {:?}", tail.next.borrow().upgrade());
}
