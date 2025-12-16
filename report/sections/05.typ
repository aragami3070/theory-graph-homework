= Обходы графа II
== Условие
Проверить, является ли граф деревом, или лесом, или не является ни тем, ни
другим.
== Код (фрагменты кода)
#set text(size: 12pt)
```rust
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn dfs<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: &u32,
    visited: &mut HashSet<u32>,
    components: &mut Vec<Index>,
) {
    if !visited.contains(start) {
        // Добавялем вершину в список компонентов связности
        components.push((*start).into());
    }
    // Отмечаем вершину как посещенную
    visited.insert(*start);
    // Получаем ребра данной вершины
    if let Some(adjacency) = graph.get_adjacency(&(*start).into()) {
        for neighbor in adjacency {
            if visited.get(&neighbor.node.number).is_none() {
                dfs(graph, &neighbor.node.number, visited, components);
            }
        }
    }
}
fn is_a_forest<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<bool> {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut subgraphs: Vec<Graph<T>> = Vec::new();
    // Разбиваем граф на подграфы по связности
    for (ind, _) in graph {
        if !visited.contains(ind) {
            let mut compnents = Vec::new();
            dfs(graph, ind, &mut visited, &mut compnents);
            subgraphs.push(graph.create_subgraph(compnents, true)?);
        }
    }
    // Проверяем, что все из них деревья
    let mut is_forest = true;
    for subgraph in subgraphs {
        is_forest = is_a_tree(&subgraph)?;
        if !is_forest {
            break;
        }
    }
    Ok(is_forest)
}
// Проверяет есть ли цикл в графе из вершины start
fn graph_have_cycle<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: &u32,
    visited: &mut HashMap<Index, ColorNode>,
) -> Result<bool> {
    if let Some(color) = visited.get_mut(&(*start).into()) {
        // Если мы заходим в серую вершину, то нашли цикл
        if *color == ColorNode::Gray {
            return Ok(true);
        }
        // Перекрашиваем вершину в серую
        *color = ColorNode::Gray;
    } else {
        return Err(Box::new(GraphError::new(
            GraphKindError::NodeNotFound,
            "Граф пустой",
        )));
    };
    if let Some(adjacency) = graph.get_adjacency(&(*start).into()) {
        // Проходимся по всем смежным вершинам
        for neighbor in adjacency {
            if let Some(color) = visited.get(&neighbor.node.number) {
                // Если цвет не серый
                if *color != ColorNode::Gray {
                    if graph_have_cycle(graph, &neighbor.node.number, visited)? {
                        return Ok(true);
                    }
                } else {
                    return Ok(true);
                }
            }
        }
    }
    if let Some(node_index) = visited.get_mut(&(*start).into()) {
        // Перекрашиваем вершину в черную
        *node_index = ColorNode::Black
    }
    Ok(false)
}
// Проверка является ли граф деревом
fn is_a_tree<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<bool> {
    let mut count = 0;
    let mut cycle = false;
    let mut is_connected = false;
    let mut visited = graph.get_nodes_with_color();
    for (index, adjacency) in graph {
        count += adjacency.len();
        if !cycle {
            cycle = graph_have_cycle(graph, index, &mut visited)?;
            if !is_connected {
                is_connected = visited.values().all(|color| *color == ColorNode::Black);
            }
        }
    }
    Ok((graph.len() as i64 - count as i64) == 1 && !cycle && is_connected)
}
// Проверка является ли граф деревом, лесом или обычным
pub fn task_5_18<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<GraphType> {
    if !graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть орграф",
        )));
    }
    if graph.len() == 0 || is_a_tree(graph)? {
        return Ok(GraphType::Tree);
    }
    if is_a_forest(graph)? {
        return Ok(GraphType::Forest);
    }
    Ok(GraphType::Default)
}
```

#set text(size: 14pt)
== Краткое описание алгоритма
Данный алгоритм классифицирует ориентированный граф по типу: дерево, лес или обычный граф, используя DFS с раскраской вершин.

=== Что делает
Определяет структурный тип графа на основе связности и наличия циклов:
- Дерево --- связный ацикличный граф (|V| = 1 компонента)
- Лес --- набор деревьев (множество ацикличных компонент связности)
- Обычный граф --- содержит циклы или несвязные компоненты с циклами

// Шаги алгоритма:
// 1. Проверка условий
//   - Работает только для ориентированных графов
//   - Возвращает ошибку для неориентированных графов
// 2. Проверка на дерево (is_a_tree)
//   - Инициализирует все вершины белыми (get_nodes_with_color)
//   - Подсчитывает общее количество рёбер
//   - Для каждой вершины запускает DFS с раскраской:
//     - Белая $->$ Серая (в обработке)
//     - Серая $->$ Чёрная (обработана)
//     - Повторное посещение серой = цикл
//     - Проверяет условия дерева: $|V| - |E| = 1$ и без циклов и все чёрные
// 3. Проверка на лес (is_a_forest)
//   - DFS находит все компоненты связности
//   - Создаёт подграфы для каждой компоненты (create_subgraph)
//   - Проверяет, что ВСЕ компоненты --- деревья
// 4. Иерархическая классификация
//   - Пустой граф $->$ Tree
//   - Связный ацикличный $->$ Tree
//   - Несколько деревьев $->$ Forest
//   - Иначе $->$ Default
// 5. DFS с раскраской (graph_have_cycle)
//   - Рекурсивно обходит граф с 3 состояниями вершин
//   - Обнаруживает back edges (серые вершины) $->$ цикл
//   - Завершает обработку чёрными вершинами
//
== Примеры входных и выходных данных

=== Входные данные
```
"2": [(7, 4), (3, 1), (5, 8)],
"3": [(6, 20), (8, 62)],
"4": [],
"5": [],
"6": [],
"7": [(90, 100)],
"8": [],
"90": [(4, 7)]
```
#image("images/03.png", height: 30%)

=== Выходные данные
```
Данный граф --- дерево
```
