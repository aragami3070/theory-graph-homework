use std::{collections::HashSet, error::Error, fmt::Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn has_cycle<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    current: u32,
    parent: Option<u32>,
    visited: &mut HashSet<u32>,
) -> bool {
	// Добавляем вершину в посещенные
    visited.insert(current);

	// Берем все ребра из данной вершины
    if let Some(adj) = graph.get_adjacency(&current) {
        for edge in adj {
            let neighbor = edge.node.number;
			// Если смежная вершина не посещена, то проверяем а цикл
			// Если посещена, и не является
            if !visited.contains(&neighbor) {
                if has_cycle(graph, neighbor, Some(current), visited) {
                    return true;
                }
            } else if Some(neighbor) != parent {
                return true; // Цикл найден
            }
        }
    }
    false
}
