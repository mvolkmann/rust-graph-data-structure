use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::vec::Vec;

// This is a common combination of types
// for holding a reference to a value that can be mutated.
// Rc cannot mutate what it holds, but
// RefCell provides "interior mutability" which
// allows a mutable borrow while immutable borrows exist.
// Normally this is not allowed by the compiler, but using
// RefCell moves the checking of correct usage to runtime.
type Wrapper<T> = Rc<RefCell<Node<T>>>;

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
        Node { data, children: vec![] }
    }

    fn depth_first(&self) {
        println!("{}", self.data);
        for child in &self.children {
            //let child_node = child.borrow();
            //child_node.depth_first();
            child.borrow().depth_first();
        }
    }

    fn wrap(data: T) -> Wrapper<T> {
        Rc::new(RefCell::new(Node::new(data)))
    }
}

fn main() {
    let a = Node::wrap('A');
    let b = Node::wrap('B');
    let c = Node::wrap('C');
    let d = Node::wrap('D');

    a.borrow_mut().add_child(Rc::clone(&b));
    a.borrow_mut().add_child(Rc::clone(&c));
    b.borrow_mut().add_child(Rc::clone(&d));
    a.borrow().depth_first();
}