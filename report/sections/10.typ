= Веса IVc
== Условие
Определить, есть ли в графе вершина, каждая из минимальных стоимостей пути от
которой до остальных не превосходит $N$
== Код (фрагменты кода)
#set text(size: 12pt)
```rust
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn floid_uorshel<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    pos_for_index: &HashMap<Index, usize>,
) -> Vec<Vec<u32>> {
    let mut dist = vec![vec![u32::MAX; graph.len()]; graph.len()];

    for (i, dst) in dist.iter_mut().enumerate() {
        dst[i] = 0;
    }

    // Заполняем dist минимальным расстоянием из одной вершины в другую
    for (ind_from, adj) in graph.iter() {
        let from = pos_for_index[ind_from];
        for edge in adj {
            let to = pos_for_index[&edge.node.number];
            dist[from][to] = dist[from][to].min(edge.weight);
        }
    }

    // Проходимся по всем вершинам и находим более короткие пути,
    // через другие вершины
    for middle in 0..graph.len() {
        for from in 0..graph.len() {
            if dist[from][middle] == u32::MAX {
                continue;
            }
            for to in 0..graph.len() {
                if dist[middle][to] == u32::MAX {
                    continue;
                }
                // Если через middle путь короче, то обновляем dist[from][to]
                let through_middle = dist[from][middle] + dist[middle][to];
                if through_middle < dist[from][to] {
                    dist[from][to] = through_middle;
                }
            }
        }
    }
    dist
}

/// Найти в [`Graph<T>`] вершину, каждая из минимальных стоимостей пути от
/// которой до остальных не превосходит limit.
///
/// # Errors
/// Эта функция вернет ошибку, если граф ориентированный.
/// Ошибка типа: [`GraphError`]
pub fn task_10_3<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    limit: &u32,
) -> Result<i32> {
    if graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть неориентированный граф",
        )));
    }

    // HashMap для сопостовления индекса вершины с индексом в dist
    let pos_for_index: HashMap<Index, usize> = graph
        .iter()
        .enumerate()
        .map(|(i, (&ind, _))| (ind, i))
        .collect();

    let dist = floid_uorshel(graph, &pos_for_index);

    'nodes: for (node_ind, cur_dists) in dist.iter().enumerate() {
        for (edge_ind, weight) in cur_dists.iter().enumerate() {
            if node_ind == edge_ind {
                continue;
            }
            // Если путь слишком длинный, то node_ind не подходит
            if weight > limit {
                continue 'nodes;
            }
        }

        // Если вершина подошла
        return Ok(
            match pos_for_index
                .iter()
                .find_map(|(key, &val)| if val == node_ind { Some(key) } else { None })
            {
                Some(key) => key.0 as i32,
                None => {
                    return Err(Box::new(GraphError::new(
                        GraphKindError::NodeNotFound,
                        "что-то пошло не так",
                    )));
                }
            },
        );
    }

    Ok(-1)
}
```

#set text(size: 14pt)
== Краткое описание алгоритма
Данный алгоритм реализует алгоритм Флойда-Уоршелла для поиска вершины, из
которой все кратчайшие пути до остальных вершин не превышают заданный лимит в
неориентированном графе.

=== Что делает
Находит вершину start, для которой max_distance(start, v) $lt.eq$ limit
для всех v $eq.not$ start:
- Вычисляет матрицу всех кратчайших путей между парами вершин
- Проверяет каждую стартовую вершину на выполнение условия
- Возвращает индекс первой подходящей вершины или -1

Шаги алгоритма:
1. Проверка условий
  - Работает только для неориентированных графов
  - Возвращает ошибку для ориентированных графов
2. Создание индексации вершин
  - pos_for_index: HashMap\<Index, usize\> сопоставляет индексы графа с
      позициями в матрице
3. Флойд-Уоршелл (floid_uorshel)
  - Инициализирует матрицу dist[N][N] с MAX, диагональ 0
  - Заполняет прямые ребра: dist[from][to] = min(dist[from][to], edge.weight)
  - |V| итераций: для каждой промежуточной вершины middle:

    dist[from][to] = min(dist[from][to], dist[from][middle] + dist[middle][to])

4. Проверка условий для каждой вершины
  - Для строки dist[node_ind] проверяет все weight $lt.eq$ limit
    (кроме диагонали)
  - continue 'nodes при первом превышении лимита
5. Обратное сопоставление индекса
  - Находит Index по позиции в матрице через pos_for_index
  - Возвращает index для первой подходящей вершины
6. Результат
  - Индекс подходящей вершины или -1, если ни одна не удовлетворяет условию

== Примеры входных и выходных данных

=== Входные данные
```
```

=== Выходные данные
```
```

