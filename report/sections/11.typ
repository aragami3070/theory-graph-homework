= Список смежности IVa
== Условие
Решить задачу на нахождение максимального потока любым алгоритмом.
== Код (фрагменты кода)
```rust
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fmt::Debug,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError, Index};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Создаем HashMap-ы для пропускных способностей и потоков
fn build_capacity_and_flow<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> (HashMap<(Index, Index), u32>, HashMap<(Index, Index), i32>) {
    let mut capacity: HashMap<(Index, Index), u32> = HashMap::new();

    let mut flow: HashMap<(Index, Index), i32> = HashMap::new();

    for (&from, adj) in graph.iter() {
        for edge in adj {
            *capacity.entry((from, edge.node.number)).or_insert(0) += edge.weight;
            // Обнуляем поток по всем ребрам
            flow.entry((from, edge.node.number)).or_insert(0);
            flow.entry((edge.node.number, from)).or_insert(0);
        }
    }

    (capacity, flow)
}

/// Остаточная пропускная способность от (from, to)
fn residual(
    capacity: &HashMap<(Index, Index), u32>,
    flow: &HashMap<(Index, Index), i32>,
    from: Index,
    to: Index,
) -> u32 {
    let c = *capacity.get(&(from, to)).unwrap_or(&0);
    let f = *flow.get(&(from, to)).unwrap_or(&0);
    if c == 0 && f > 0 {
        // чисто обратное ребро: остаточная = f
        f as u32
    } else {
        // прямое ребро: остаточная = c - f
        c.saturating_sub(f.max(0) as u32)
    }
}

/// Поиск увеличивающего пути BFS-ом.
fn bfs_find_path<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    capacity: &HashMap<(Index, Index), u32>,
    flow: &HashMap<(Index, Index), i32>,
    start: Index,
    end: Index,
) -> (bool, HashMap<Index, Option<Index>>) {
    let mut parent: HashMap<Index, Option<Index>> = HashMap::new();
    let mut visited: HashMap<Index, bool> = HashMap::new();

    let mut queue = VecDeque::<Index>::new();

    queue.push_back(start);
    parent.insert(start, None);
    visited.insert(start, true);

    'bfs: while let Some(from_ind) = queue.pop_front() {
        // Прямые рёбра
        if let Some(adj) = graph.get_adjacency(&from_ind) {
            for edge in adj {
                let to_ind = edge.node.number;
                if !visited.get(&to_ind).copied().unwrap_or(false)
                    && residual(capacity, flow, from_ind, to_ind) > 0
                {
                    parent.insert(to_ind, Some(from_ind));
                    visited.insert(to_ind, true);
                    if to_ind == end {
                        break 'bfs;
                    }
                    queue.push_back(to_ind);
                }
            }
            // Обратные ребра
            for (&(from, to), f_from_to) in flow.iter() {
                if to == from_ind && *f_from_to > 0 && !visited.get(&from).copied().unwrap_or(false)
                {
                    parent.insert(from, Some(from_ind));
                    visited.insert(from, true);
                    if to == end {
                        break 'bfs;
                    }

                    queue.push_back(to);
                }
            }
        }
    }
    (parent.contains_key(&end), parent)
}

/// Нахождение максимального потока
pub fn task_11<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: Index,
    end: Index,
) -> Result<u32> {
    if !graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть сеть (орграф)",
        )));
    }
    let (capacity, mut flow) = build_capacity_and_flow(graph);
    let mut max_flow: i32 = 0;

    loop {
        let (found, parent) = bfs_find_path(graph, &capacity, &flow, start, end);
        if !found {
            break;
        }

        // Находим бутылочное горлышко
        let mut path_flow: i32 = i32::MAX;
        let mut v = end;
        while let Some(Some(u)) = parent.get(&v) {
            let r = residual(&capacity, &flow, *u, v);
            path_flow = path_flow.min(r as i32);
            v = *u;
        }

        // Обновляем потоки вдоль пути
        let mut v2 = end;
        while let Some(Some(u)) = parent.get(&v2) {
            *flow.entry((*u, v2)).or_insert(0) += path_flow;
            *flow.entry((v2, *u)).or_insert(0) -= path_flow;
            v2 = *u;
        }

        max_flow += path_flow;
    }

    Ok(max_flow as u32)
}
```

== Краткое описание алгоритма
Данный алгоритм реализует алгоритм Эдмондса-Карпа для нахождения максимального
потока в сети (ориентированном взвешенном графе).

=== Что делает
Находит максимальный поток от источника `start` до стока `end` в транспортной сети:
- Строит остаточную сеть с прямыми и обратными ребрами
- Ищет увеличивающие пути BFS-ом до отсутствия таких путей
- Возвращает суммарный поток через все найденные пути

Шаги алгоритма:
1. Проверка условий
  - Работает только для ориентированных графов (сетей)
  - Возвращает ошибку для неориентированных графов
2. Инициализация структур
  - `capacity[(u,v)]` --- пропускная способность ребра
  - `flow[(u,v)]` --- текущий поток по ребру (0 изначально)
3. Основной цикл Эдмондса-Карпа
  - Повторяет до отсутствия увеличивающего пути
  - BFS ищет путь от `start` до `end` в остаточной сети
4. Поиск увеличивающего пути BFS (`bfs_find_path`)
  - Строит дерево достижимости с `parent`
  - Проверяет прямые ребра: `residual(u,v) > 0`
  - Проверяет обратные ребра: `flow(v,u) > 0`
5. Остаточная пропускная способность (`residual`)
  - Прямое ребро: `c(u,v) - flow(u,v)`
  - Обратное ребро: `flow(v,u)`
6. Обновление потока по пути
  - Находит бутылочное горлышко `path_flow = min(residual)`
  - `flow(u,v) += path_flow`, `flow(v,u) -= path_flow`
  - `max_flow += path_flow`
7. Результат
  - Возвращает суммарный максимальный поток `max_flow`

=== Входные данные
```
```

=== Выходные данные
```
```

