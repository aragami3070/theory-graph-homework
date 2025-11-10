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
        if let Some(adjacency) = graph.get_adjacency(&current) {
            for neighbor in adjacency {
                let neighbor_index = neighbor.node.number;
                // Если смежная вершина не посещена, то добавляем в очередь
                // и отмечаем как посещенную
                if !visited.contains(&neighbor_index) {
                    visited.insert(neighbor_index);
                    queue.push_back(neighbor_index);
                }
            }
        }
    }
}

