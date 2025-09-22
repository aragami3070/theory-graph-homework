mod graph;
use graph::core;
fn main() {
    let node_a = core::Node::<u32>::new(1, 20);
    let node_b = core::Node::<u32>::new(2, 23);
    let node_c = core::Node::<u32>::new(3, 23);

    let edge = core::Edge::<u32>::new(&node_a.number, 140, &node_a.value);
    let edge2 = core::Edge::<u32>::new(&node_b.number, 109, &node_b.value);
    let edge3 = core::Edge::<u32>::new(&node_c.number, 10, &node_c.value);

    let adjacency = core::Adjacency::<u32>::new(edge2);
    let mut adjacency_list = core::AdjacencyList::<u32>::new(1, adjacency, true);

    adjacency_list.add_node(node_b.number);
    adjacency_list.add_edge(node_b.clone(), edge);
    adjacency_list.add_node(node_c.number);
    adjacency_list.add_edge(node_b, edge3);
    println!("{adjacency_list}")
}
