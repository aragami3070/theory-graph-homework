use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Debug,
};

use serde::{Serialize, de::DeserializeOwned};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
use crate::graph::core::{Graph, GraphError, GraphKindError};

fn bfs<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: &u32,
    visited: &mut HashSet<u32>,
) {
    // Создали очередь со стартовой вершиной
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(*start);
    visited.insert(*start);

    // Пока в очереди есть вершины
    while let Some(current) = queue.pop_front() {
        if let Some(adjacency) = graph.get_adjacency(&current.into()) {
            for neighbor in adjacency {
                let neighbor_index = neighbor.node.number;
                // Если смежная вершина не посещена, то добавляем в очередь
                // и отмечаем как посещенную
                if !visited.contains(&neighbor_index) {
                    visited.insert(*neighbor_index);
                    queue.push_back(*neighbor_index);
                }
            }
        }
    }
}

// Выясняет, является ли граф связным.
pub fn task_6_4<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<bool> {
    if graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть неориентированный граф",
        )));
    }

    let mut visited: HashSet<u32> = HashSet::new();

    if let Some(node_ind) = graph.get_some_node_index() {
        bfs(graph, node_ind, &mut visited);
    }

    Ok(visited.len() == graph.len())
}
