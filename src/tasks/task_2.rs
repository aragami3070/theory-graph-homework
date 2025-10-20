use crate::graph::core::Graph;

/// Получить полустепень захода данной вершины орграфа
pub fn task_2_4<T: Clone>(graph: &Graph<T>, node_index: &u32) -> u32 {
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
    half_step
}
