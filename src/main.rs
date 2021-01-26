use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::vec::Vec;

// Lifetime annotations must precede generic types in the angle brackets.
#[derive(Debug)]
struct Node<'a, T> {
    data: T,
    children: RefCell<Vec<&'a Node<'a, T>>>
}

impl<'a, T: Display> Node<'a, T> {
    fn add_child(&mut self, child: &'a Rc<Node<'a, T>>) {
        self.children.borrow_mut().push(child);
    }

    fn new(data: T) -> Node<'a, T> {
        Node { data, children: RefCell::new(Vec::new()) }
    }

    fn depth_first(&self) {
        println!("node {}", self.data);
        for child in self.children.borrow().iter() {
            child.depth_first();
        }
    }
}

fn main() {
    let mut a: Rc<Node<char>> = Rc::new(Node::new('A'));
    let mut b: Rc<Node<char>> = Rc::new(Node::new('B'));
    let c: Rc<Node<char>> = Rc::new(Node::new('C'));
    let d: Rc<Node<char>> = Rc::new(Node::new('D'));

    a.add_child(&Rc::clone(&b));
    a.add_child(&Rc::clone(&c));
    b.add_child(&Rc::clone(&d));
    a.depth_first();
}
