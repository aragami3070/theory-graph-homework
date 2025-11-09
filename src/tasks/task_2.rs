use std::{error::Error, fmt::Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError};

/// Получить полустепень захода данной вершины орграфа
pub fn task_2_4<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    node_index: &u32,
) -> Result<u32, Box<dyn Error>> {
    if !graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть орграф",
        )));
    }
    // Полустепень вершины
    let mut half_step: u32 = 0;
    // Проходимся по всем вершинам и считаем сколько раз встречается
    // эта вершина в ребрах
    for (_, adjacency) in graph {
        for edg in adjacency {
            if edg.node.number == *node_index {
                half_step += 1;
                break;
            }
        }
    }
    Ok(half_step)
}
