use crate::graph::core::Graph;

/// Получить степень каждой вершины орграфа
pub fn task_3_5<T: Clone>(graph: &Graph<T>) -> Vec<(u32, u32)> {
    // Вектор пар (вершина, степень вершины)
    let mut result: Vec<(u32, u32)> = Vec::new();
    for (&node_index, adjacency) in graph {
        result.push((node_index, adjacency.len().try_into().unwrap()));
    }
    result
}
