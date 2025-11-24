use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fmt::Debug,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn deikstra<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: u32,
    destination: u32,
    weight_limit: u32,
) -> bool {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    // Для всех вершин кроме начальной выставляем максимум
    for (index, _) in graph.iter() {
        dist.insert(*index, std::u32::MAX);
    }

    // Ставим 0 для начальной вершины
    dist.insert(start, 0);
    heap.push((0u32, start));

    while let Some((weight, cur_node)) = heap.pop() {
        // Если длина уже больше weight_limit, то нет смысла считать дальше
        if weight > weight_limit {
            break;
        }

        //  Если дошли до
        if cur_node == destination {
            return true;
        }

        if let Some(adj) = graph.get_adjacency(&cur_node) {
            for edge in adj {
                let node = edge.node.number;
                let new_weight = weight + edge.weight;
                if new_weight < dist[&node] && new_weight <= weight_limit {
                    dist.insert(node, new_weight);
                    heap.push((new_weight, node));
                }
            }
        }
    }

    false
}

pub fn task_8_1<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: u32,
    destination: u32,
    weight_limit: u32,
) -> Result<bool> {
    if graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть неориентированный граф",
        )));
    }

    Ok(deikstra(graph, start, destination, weight_limit))
}
