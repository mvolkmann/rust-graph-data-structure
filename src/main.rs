use std::fmt::Display;
use std::vec::Vec;

// Lifetime annotations must precede generic types in the angle brackets.
#[derive(Debug)]
struct Node<'a, T> {
    data: T,
    children: Vec<&'a Node<'a, T>>
}

impl<'a, T: Display> Node<'a, T> {
    fn add_child(&mut self, child: &'a Node<T>) {
        self.children.push(child);
    }

    fn new(data: T) -> Node<'a, T> {
        Node { data, children: Vec::new() }
    }

    fn depth_first(&self) {
        println!("node {}", self.data);
        for child in &self.children {
            child.depth_first();
        }
    }
}

fn main() {
    let mut a: Node<char> = Node::new('A');
    let mut b: Node<char> = Node::new('B');
    let c: Node<char> = Node::new('C');
    let d: Node<char> = Node::new('D');

    a.add_child(&b);
    a.add_child(&c);
    b.add_child(&d);
    a.depth_first();
}
