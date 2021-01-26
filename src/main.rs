use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::vec::Vec;

// This is a common combination of types.
// Rc can't mutate what it holds.
// RefCell provides "interior mutability" which
// allows a mutable borrow while immutable borrows exist.
// Normally this is allowed by the compiler, but using RefCell
// moves the checking of correct usage to runtime.
type Wrapper<T> = Rc<RefCell<Node<T>>>;

fn wrap<T: Display>(data: T) -> Wrapper<T> {
    Rc::new(RefCell::new(Node::new(data)))
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    children: Vec<Wrapper<T>>
}

impl<T: Display> Node<T> {
    fn add_child(&mut self, child: Wrapper<T>) {
        self.children.push(child);
    }

    fn new(data: T) -> Node<T> {
        Node { data, children: Vec::new() }
    }

    fn depth_first(&self) {
        println!("node {}", self.data);
        for child in &self.children {
            child.borrow().depth_first();
        }
    }
}

fn main() {
    let a = wrap('A');
    let b = wrap('B');
    let c = wrap('C');
    let d = wrap('D');

    a.borrow_mut().add_child(Rc::clone(&b));
    a.borrow_mut().add_child(Rc::clone(&c));
    b.borrow_mut().add_child(Rc::clone(&d));
    a.borrow_mut().depth_first();
}