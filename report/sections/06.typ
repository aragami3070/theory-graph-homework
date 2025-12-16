= Обходы графа II
== Условие
Выяснить, является ли граф связным.
== Код (фрагменты кода)
#set text(size: 12pt)
```rust
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn bfs<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: &u32,
    visited: &mut HashSet<u32>,
) {
    // Создали очередь со стартовой вершиной
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(*start);
    visited.insert(*start);
    // Пока в очереди есть вершины
    while let Some(current) = queue.pop_front() {
        if let Some(adjacency) = graph.get_adjacency(&current.into()) {
            for neighbor in adjacency {
                let neighbor_index = neighbor.node.number;
                // Если смежная вершина не посещена, то добавляем в очередь
                // и отмечаем как посещенную
                if !visited.contains(&neighbor_index) {
                    visited.insert(*neighbor_index);
                    queue.push_back(*neighbor_index);
                }
            }
        }
    }
}
/// Выясняет, является ли граф связным.
pub fn task_6_4<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<bool> {
    if graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть неориентированный граф",
        )));
    }
    let mut visited: HashSet<u32> = HashSet::new();
    if let Some(node_ind) = graph.get_some_node_index() {
        bfs(graph, node_ind, &mut visited);
    }
    Ok(visited.len() == graph.len())
}
```
#set text(size: 14pt)
== Краткое описание алгоритма
Данный алгоритм проверяет связность неориентированного графа с помощью BFS
обхода от произвольной вершины.

=== Что делает
Проверяет, является ли граф связным --- все вершины достижимы друг из друга:
- Выполняет BFS от одной стартовой вершины
- Сравнивает количество посещенных вершин с общим числом вершин
- Возвращает true, если граф связный, false --- иначе

// Шаги алгоритма:
// 1. Проверка условий
//   - Работает только для неориентированных графов
//   - Возвращает ошибку для ориентированных графов
// 2. Инициализация BFS
//   - Берет произвольную стартовую вершину через get_some_node_index()
//   - Создает множество посещенных вершин
// 3. BFS обход
//   - Инициализирует очередь с стартовой вершиной
//   - Помечает стартовую вершину как посещенную
//   - Обрабатывает вершины по принципу FIFO
// 4. Обработка соседей
//   - Для текущей вершины получает все смежности
//   - Для каждого непосещенного соседа:
//     - Помечает как посещенный
//     - Добавляет в очередь
// 5. Проверка связности
//   - Сравнивает размер множества посещенных с graph.len()
//   - visited.len() == graph.len() --- граф связный

== Примеры входных и выходных данных

=== Входные данные
```
"1": [(6, 23), (2, 1), (1, 15)],
"2": [(2, 20), (1, 1)],
"3": [],
"5": [(6, 2)],
"6": [(1, 23), (5, 2)],
"7": [],
"8": []
```

=== Выходные данные
```
Данный граф: не связен
```
