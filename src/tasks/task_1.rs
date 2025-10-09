use crate::graph::core::Graph;

pub fn task_1_4<T: Clone>(graph: &Graph<T>, node_index: &u32) -> u32 {
    let mut half_step: u32 = 0;
    for (node_ind, adjacency) in graph {
        if node_ind == node_index {
            continue;
        }
        for edg in adjacency {
            if edg.node.number == *node_index {
                half_step += 1;
                break;
            }
        }
    }
    half_step
}
