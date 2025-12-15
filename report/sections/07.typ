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

Шаги алгоритма:
1. Проверка условий
  - Работает только для неориентированных графов
  - Возвращает ошибку для ориентированных графов
2. Сбор всех рёбер
  - Получает все ребра графа через get_all_edges()
  - Автоматически сортирует ребра по возрастанию веса
3. Инициализация MST
  - Создаёт новый неориентированный граф
  - Добавляет все вершины исходного графа
4. Построение MST
  - Проходит по отсортированным рёбрам по порядку веса
  - Для каждого ребра временно добавляет его в MST
  - Проверяет наличие цикла с помощью рекурсивного DFS
5. Проверка циклов (DFS с parent)
  - Рекурсивно обходит граф, отслеживая родителя
  - Если встречает посещённую вершину (не родителя) --- цикл
  - Удаляет ребро, создающее цикл
6. Результат
  - Возвращает MST как новый граф с минимальным весом

== Примеры входных и выходных данных

=== Входные данные
```
```

=== Выходные данные
```
```
