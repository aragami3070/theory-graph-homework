= Каркас III
== Условие
Найти каркас минимального веса в неориентированном графе.
== Код (фрагменты кода)
#set text(size: 12pt)

```rust
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn has_cycle<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    current: Index,
    parent: Option<Index>,
    visited: &mut HashSet<Index>,
) -> bool {
    // Добавляем вершину в посещенные
    visited.insert(current);
    // Берем все ребра из данной вершины
    if let Some(adj) = graph.get_adjacency(&current) {
        for edge in adj {
            let neighbor = edge.node.number;
            // Если смежная вершина не посещена, то проверяем а цикл
            // Если посещена, и не является родителем, то найден цикл
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
pub fn task_7_kraskal<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<Graph<T>> {
    if graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть неориентированный граф",
        )));
    }
    // Отсортированный список ребер (список смежности)
    let edges_list = graph.get_all_edges();
    let mut new_graph: Graph<T> = Graph::default();
    new_graph = new_graph.to_not_directed()?;
    // Заполняем остов вершинами
    for (index, _) in edges_list.iter() {
        if let Some(node) = graph.get_node(index)
            && new_graph.get_node(index).is_none()
        {
            new_graph.add_node(node.clone())?
        }
    }
    // Для каждого ребра из отсортированного списка пытаемся добавить без образования цикла
    for (index, edge) in edges_list.iter() {
        if let Some(node) = graph.get_node(index) {
            // Добавляем ребро
            new_graph.add_edge(node, edge)?;
            // Проверяем нет ли цикла
            let mut visited = HashSet::new();
            if has_cycle(&new_graph, *index, None, &mut visited) {
                // Если цикл, значит убираем ребро
                new_graph.delete_edge(node, &edge.node.number)?;
            }
        }
    }
    Ok(new_graph)
}
```

#set text(size: 14pt)
== Краткое описание алгоритма
Данный алгоритм реализует алгоритм Краскала для построения минимального
остовного дерева (MST) неориентированного взвешенного графа.

=== Что делает
Находит минимальное остовное дерево --- подграф, который:
- Содержит все вершины исходного графа
- Является деревом (связный и ацикличный)
- Имеет минимальный суммарный вес рёбер

// Шаги алгоритма:
// 1. Сбор всех рёбер
//   - Получает все ребра графа через get_all_edges()
//   - Автоматически сортирует ребра по возрастанию веса
// 2. Инициализация MST
//   - Создаёт новый неориентированный граф
//   - Добавляет все вершины исходного графа
// 3. Построение MST
//   - Проходит по отсортированным рёбрам по порядку веса
//   - Для каждого ребра временно добавляет его в MST
//   - Проверяет наличие цикла с помощью рекурсивного DFS
// 4. Проверка циклов (DFS с parent)
//   - Рекурсивно обходит граф, отслеживая родителя
//   - Если встречает посещённую вершину (не родителя) --- цикл
//   - Удаляет ребро, создающее цикл
// 5. Результат
//   - Возвращает MST как новый граф с минимальным весом
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
#image("images/04.png", height: 30%)

=== Выходные данные
```
2: [
  Edge: {
    Number: 4,
    Weight: 2,
    Value: D
  },
  Edge: {
    Number: 1,
    Weight: 7,
    Value: A
  },
],
4: [
  Edge: {
    Number: 2,
    Weight: 2,
    Value: B
  },
  Edge: {
    Number: 3,
    Weight: 6,
    Value: C
  },
  Edge: {
    Number: 6,
    Weight: 9,
    Value: F
  },
],
1: [
  Edge: {
    Number: 2,
    Weight: 7,
    Value: B
  },
],
5: [
  Edge: {
    Number: 3,
    Weight: 9,
    Value: C
  },
],
6: [
  Edge: {
    Number: 4,
    Weight: 9,
    Value: D
  },
],
3: [
  Edge: {
    Number: 4,
    Weight: 6,
    Value: D
  },
  Edge: {
    Number: 5,
    Weight: 9,
    Value: E
  },
],
```
