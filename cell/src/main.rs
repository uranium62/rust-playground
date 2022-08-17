use std::cell::{RefCell};
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<_Node<T>>>;

struct _Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}

struct Node<T>(NodeRef<T>);

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node(Rc::new(RefCell::new(_Node {
            inner_value: value,
            adjacent: Vec::new()
        })))
    }
    
    fn add_adjacent(&self, other: &Node<T>) {
        self.0.borrow_mut().adjacent.push(other.0.clone())
    }
}

struct Graph<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Graph<T> {
    fn with_nodes(nodes: Vec<Node<T>>) -> Self {
        Self { nodes }
    }
}


fn main() {
    // Create some nodes
    let node_1 = Node::new(1);
    let node_2 = Node::new(2);
    let node_3 = Node::new(3);

    // Connect some of the nodes (with directed edges)
    node_1.add_adjacent(&node_2);
    node_1.add_adjacent(&node_3);
    node_2.add_adjacent(&node_1);
    node_3.add_adjacent(&node_1);

    // Add nodes to graph
    let graph = Graph::with_nodes(vec![node_1, node_2, node_3]);

    // Show every node in the graph and list their neighbors
    for node in graph.nodes.iter().map(|n| n.0.borrow()) {
        let value = node.inner_value;
        let neighbours = node.adjacent.iter()
            .map(|n| n.borrow().inner_value)
            .collect::<Vec<_>>();
        println!("node ({}) is connected to: {:?}", value, neighbours);
    }
}