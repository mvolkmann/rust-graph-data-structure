#[derive(Debug)]
struct Node<T> {
    data: T
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data }
    }
}

// Lifetime annotations must precede generic types in the angle brackets.
#[derive(Debug)]
struct Edge<'a, T> {
    from: &'a Node<T>,
    to: &'a Node<T>,
}
impl<'a, T> Edge<'a, T> {
    fn new(from: &'a Node<T>, to: &'a Node<T>) -> Edge<'a, T> {
        Edge { from, to }
    }
}

fn main() {
    let a: Node<char> = Node::new('A');
    let b: Node<char> = Node::new('B');
    let c: Node<char> = Node::new('C');
    let d: Node<char> = Node::new('D');
    let e1: Edge<char> = Edge::new(&a, &b);
    let e2: Edge<char> = Edge::new(&a, &c);
    let e3: Edge<char> = Edge::new(&b, &d);
    dbg!(e1, e2, e3);
}
