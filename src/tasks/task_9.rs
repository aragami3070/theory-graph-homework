use std::{collections::HashMap, error::Error, fmt::Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError, Index};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Находим сумму минимальных путей до всех вершин достижимых вершин из
/// вершины start
fn bellman_ford<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: &Index,
) -> u32 {
    // Создаем HashMap из индекса вершины и длины пути до нее
    // (изначально максимум для u32)
    let mut dist = HashMap::new();
    for (index, _) in graph.iter() {
        dist.insert(*index, u32::MAX);
    }

    // Для вершины start длина пути 0
    dist.insert(*start, 0);

    // Перебираем все ребра
    for _ in 0..graph.len() - 1 {
        for (ind, adj) in graph.iter() {
            for edge in adj {
                let edge_node_num = edge.node.number;
                // Если вес ребра короче чем чем текущее значение в dist
                if dist[&edge_node_num] != u32::MAX
                    && dist[ind] > dist[&edge_node_num] + edge.weight
                {
                    // Обновляем значение веса для данной вершины
                    dist.insert(*ind, dist[&edge_node_num] + edge.weight);
                }
            }
        }
    }

    dist.iter()
        .filter(|&(ind, _)| ind != start) // не учитываем расстояние от start
        .map(|(_, weight)| *weight)
        .filter(|&v| v != u32::MAX) // не учитываем недостижимые
        .sum::<u32>()
}

/// Найти в [`Graph<T>`] вершину, минимальные стоимости путей от которой до
/// остальных в сумме не превосходят limit
///
/// # Errors
/// Эта функция вернет ошибку, если граф ориентированный.
/// Ошибка типа: [`GraphError`]
pub fn task_9_2<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    limit: &u32,
) -> Result<i32> {
    if graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть неориентированный граф",
        )));
    }

    for (start, _) in graph {
        let res = bellman_ford(graph, start);
        if res > 0 && res <= *limit {
            return Ok((**start) as i32);
        }
    }

    // Вершина не найдена
    Ok(-1)
}
