use std::{error::Error, fmt::Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError};

/// Получить степень каждой вершины орграфа
pub fn task_3_5<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    if !graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть орграф",
        )));
    }
    // Вектор пар (вершина, степень вершины)
    let mut result: Vec<(u32, u32)> = Vec::new();
    for (&node_index, adjacency) in graph {
        // заполняем вектор парами (вершина, количество ребер)
        result.push((*node_index, adjacency.len().try_into().unwrap()));
    }
    Ok(result)
}
