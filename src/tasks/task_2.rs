use crate::graph::core::Graph;

pub fn task_2_5<T: Clone>(graph: &Graph<T>) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = Vec::new();
    for (&node_index, adjacency) in graph {
        result.push((node_index, adjacency.len().try_into().unwrap()));
    }
    result
}
