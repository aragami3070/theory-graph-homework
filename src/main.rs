mod graph;
use graph::core;
fn main() {
	let edge = core::Edge::<u32>::new(1, 100, 20);
    let adjacency = core::Adjacency::<u32>::new(edge);
	let adjacency_list = core::AdjacencyList::<u32>::new(1, adjacency);
	println!("{adjacency_list}")
}
