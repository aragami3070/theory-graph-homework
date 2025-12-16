= Веса IVa
== Условие
Определить, существует ли путь длиной не более $L$ между двумя заданными
вершинами графа.
== Код (фрагменты кода)
#set text(size: 12pt)
```rust
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn deikstra<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: Index,
    destination: Index,
    weight_limit: u32,
) -> bool {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    // Для всех вершин кроме начальной выставляем максимум
    for (index, _) in graph.iter() {
        dist.insert(*index, u32::MAX);
    }

    // Ставим 0 для начальной вершины
    dist.insert(start, 0);
    heap.push((0u32, start));

    while let Some((weight, cur_node)) = heap.pop() {
        // Если длина уже больше weight_limit, то нет смысла считать дальше
        if weight > weight_limit {
            break;
        }

        // Если дошли до нужной вершины
        if cur_node == destination {
            return true;
        }

        if let Some(adj) = graph.get_adjacency(&cur_node) {
            for edge in adj {
                let node = edge.node.number;
                let new_weight = weight + edge.weight;

                // Если сосдед не посящен
                if new_weight < dist[&node] && new_weight <= weight_limit {
                    dist.insert(node, new_weight);
                    heap.push((new_weight, node));
                }
            }
        }
    }

    false
}

pub fn task_8_1<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: u32,
    destination: u32,
    weight_limit: u32,
) -> Result<bool> {
    if graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть неориентированный граф",
        )));
    }

    Ok(deikstra(
        graph,
        start.into(),
        destination.into(),
        weight_limit,
    ))
}
```

#set text(size: 14pt)
== Краткое описание алгоритма
Данный алгоритм реализует модифицированный алгоритм Дейкстры для проверки
достижения вершины с ограничением по весу пути в неориентированном графе.

=== Что делает
Проверяет, существует ли путь от стартовой вершины до целевой с весом $lt.eq$
weight_limit:
- Находит кратчайший путь с приоритетной очередью (BinaryHeap)
- Прерывает поиск при превышении weight_limit
- Возвращает true, если целевая вершина достижима в пределах лимита
//
// Шаги алгоритма:
// 1. Проверка условий
//   - Работает только для неориентированных графов
//   - Возвращает ошибку для ориентированных графов
// 2. Инициализация расстояний
//   - dist[index] = MAX для всех вершин кроме стартовой
//   - dist[start] = 0
//   - Пушит (0, start) в min-heap (BinaryHeap с инверсией)
// 3. Основной цикл Дейкстры
//   - Извлекает вершину с минимальным текущим весом
//   - Прерывает, если weight > weight_limit
//   - Возвращает true, если достигнута целевая вершина
// 4. Расслабление рёбер
//   - Для каждого соседа проверяет new_weight = weight + edge.weight
//   - Обновляет dist[neighbor], если new_weight < dist[neighbor] и ≤ limit
//   - Пушит обновлённую пару (new_weight, neighbor) в heap
// 5. Раннее завершение
//   - weight > weight_limit $->$ break
//   - cur_node == destination $->$ return true
// 6. Результат
//   - false, если не удалось достичь destination в пределах лимита
//
== Примеры входных и выходных данных

=== Входные данные
```
"1": [(2, 7), (3, 8)],
"2": [(1, 7), (3, 11), (4, 2)],
"3": [(1, 8), (2, 11), (4, 6), (5, 9)],
"4": [(2, 2), (3, 6), (5, 11), (6, 9)],
"5": [(3, 9), (4, 11), (6, 10)],
"6": [(4, 9), (5, 10)]
```

=== Выходные данные
```
Введите путь до файла (для временного графа):
assets/input10.json
Введите индекс начальной точки:
1
Введите индекс конечной точки:
6
Введите максимальную длину пути:
30
Существует
```
