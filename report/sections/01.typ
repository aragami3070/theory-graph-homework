= Минимальные требования для класса Граф
== Условие
Для решения всех задач курса необходимо создать класс (или иерархию классов - на усмотрение разработчика), содержащий:

1. Структуру для хранения списка смежности графа (не работать с графом через матрицы смежности, если в некоторых алгоритмах удобнее использовать список ребер - реализовать метод, создающий список рёбер на основе списка смежности);

2. Конструкторы (не менее 3-х):
  - конструктор по умолчанию, создающий пустой граф
  - конструктор, заполняющий данные графа из файла
  - конструктор-копию (аккуратно, не все сразу делают именно копию)
  - специфические конструкторы для удобства тестирования

3. Методы:

  - добавляющие вершину,
  - добавляющие ребро (дугу),
  - удаляющие вершину,
  - удаляющие ребро (дугу),
  - выводящие список смежности в файл (в том числе в пригодном для чтения конструктором формате).
  - Не выполняйте некорректные операции, сообщайте об ошибках.

4. Должны поддерживаться как ориентированные, так и неориентированные графы. Заранее предусмотрите возможность добавления меток и\или весов для дуг. Поддержка мультиграфа не требуется.

5. Добавьте минималистичный консольный интерфейс пользователя (не смешивая его с реализацией!), позволяющий добавлять и удалять вершины и рёбра (дуги) и просматривать текущий список смежности графа.

6. Сгенерируйте не менее 4 входных файлов с разными типами графов (балансируйте на комбинации ориентированность-взвешенность) для тестирования класса в этом и последующих заданиях. Графы должны содержать не менее 7-10 вершин, в том числе петли и изолированные вершины.

Замечание:

В зависимости от выбранного способа хранения графа могут появиться дополнительные трудности при удалении-добавлении, например, необходимость переименования вершин, если граф хранится списком $($например, vector C++, List C#$)$. Этого можно избежать, если хранить в списке пару (имя вершины, список смежных вершин), или хранить в другой структуре (например, Dictionary C#$,$ map в С++, при этом список смежности вершины может также храниться в виде словаря с ключами - смежными вершинами и значениями - весами соответствующих ребер). Идеально, если в качестве вершины реализуется обобщенный тип (generic), но достаточно использовать строковый тип или свой класс.

== код (фрагменты кода)

```rust
use std::{
    collections::{
        HashMap, HashSet,
        hash_map::{self, Entry},
        hash_set,
    },
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    hash::Hash,
    io::{BufReader, BufWriter, Write},
    ops::Deref,
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Index(pub u32);


impl PartialEq<u32> for Index
where
{
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}
impl Deref for Index {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Index(value)
    }
}

type Weight = u32;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type PairEdges<T> = (Option<Edge<T>>, Option<Edge<T>>);

#[derive(Debug)]
pub struct GraphError {
    kind: GraphKindError,
    description: String,
}

#[derive(Debug)]
pub enum GraphKindError {
    NodeAlreadyExist,
    NodeNotFound,
    GraphMustBeDirected,
}

impl GraphError {
    pub fn new(kind: GraphKindError, description: &str) -> Self {
        Self {
            kind,
            description: description.to_string(),
        }
    }
}

impl Error for GraphError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            GraphKindError::NodeAlreadyExist => {
                write!(
                    f,
                    "this node already exist in graph.\nDescription: {}",
                    &self.description
                )
            }

            GraphKindError::NodeNotFound => {
                write!(
                    f,
                    "this node not found in graph.\nDescription: {}",
                    &self.description
                )
            }

            GraphKindError::GraphMustBeDirected => {
                write!(
                    f,
                    "this graph must be directed.\nDescription: {}",
                    &self.description
                )
            }
        }
    }
}

// Node part

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node<T> {
    pub number: Index,
    pub value: T,
}

impl<T> Default for Node<T>
where
    T: Default,
    T: Clone,
{
    fn default() -> Self {
        Node::<T> {
            number: 0.into(),
            value: T::default(),
        }
    }
}

impl<T> Node<T> {
    pub fn new(index: Index, value: T) -> Self {
        Self {
            number: index,
            value,
        }
    }
}

// Edge part

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge<T>
where
    T: Clone,
{
    pub node: Node<T>,
    pub weight: Weight,
}

impl<T> PartialEq for Edge<T>
where
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.node.number == other.node.number
    }
}

impl<T> Eq for Edge<T> where T: Clone {}

impl<T> Hash for Edge<T>
where
    T: Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node.number.hash(state);
    }
}

impl<T> Default for Edge<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Edge::<T> {
            node: Node::default(),
            weight: 0,
        }
    }
}

impl<T> Display for Edge<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  Edge: {{\n    Number: {},\n    Weight: {},\n    Value: {}\n  }}",
            self.node.number, self.weight, self.node.value
        )
    }
}

impl<T> Edge<T>
where
    T: Clone,
{
    /// Creates a new [`Edge<T>`].
    pub fn new(number: &Index, weight: Weight, value: &T) -> Self {
        Self {
            weight,
            node: Node::new(*number, value.clone()),
        }
    }
}

// Adjacency part

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adjacency<T>
where
    T: Clone,
{
    edges: HashSet<Edge<T>>,
}

impl<T> Default for Adjacency<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self {
            edges: HashSet::new(),
        }
    }
}

impl<T> Display for Adjacency<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_format = String::from("[\n");
        for edge in &self.edges {
            print_format.push_str(format!("{edge},\n").as_str());
        }
        print_format.push(']');
        write!(f, "{print_format}")
    }
}

impl<T> Adjacency<T>
where
    T: Clone,
{
    /// Creates a new [`Adjacency<T>`]
    pub fn new(edge: Edge<T>) -> Self {
        let mut new_edges = HashSet::new();
        new_edges.insert(edge);
        Self { edges: new_edges }
    }

    pub fn contains(&self, edge: &Edge<T>) -> bool {
        self.edges.contains(edge)
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Add in end new [`Edge<T>`]
    fn push(&mut self, edge: Edge<T>) -> Result<()> {
        self.edges.insert(edge);
        Ok(())
    }

    fn delete(&mut self, edge_index: Index) -> Result<Option<Edge<T>>> {
        let rm_edge = self
            .edges
            .iter()
            .find(|e| e.node.number == edge_index)
            .cloned();

        if let Some(rm_e) = rm_edge {
            self.edges.remove(&rm_e);
            Ok(Some(rm_e))
        } else {
            Err(Box::new(GraphError::new(GraphKindError::NodeNotFound, "")))
        }
    }
}

pub struct AdjacencyIter<'a, T>
where
    T: Clone,
{
    inner: hash_set::Iter<'a, Edge<T>>,
}

impl<'a, T> Iterator for AdjacencyIter<'a, T>
where
    T: Clone,
{
    type Item = &'a Edge<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T> IntoIterator for &'a Adjacency<T>
where
    T: Clone,
{
    type Item = &'a Edge<T>;
    type IntoIter = AdjacencyIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        AdjacencyIter {
            inner: self.edges.iter(),
        }
    }
}

// Graph part

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph<T>
where
    T: Clone,
{
    nodes: HashMap<Index, Node<T>>,
    adjacency: HashMap<Index, Adjacency<T>>,
    is_directed: bool,
}

impl<T> Display for Graph<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_format = String::new();
        for adjacency in &self.adjacency {
            print_format.push_str(format!("{}: {},\n", adjacency.0, adjacency.1).as_str());
        }
        write!(f, "{print_format}")
    }
}

impl<T> Default for Graph<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
            is_directed: false,
        }
    }
}

impl<T> Graph<T>
where
    T: Clone + Serialize + DeserializeOwned + Debug + Default,
{
    /// Creates a new [`Graph<T>`]
    pub fn new(node: Option<Node<T>>, edge_adjacency: Adjacency<T>, is_directed: bool) -> Self {
        let mut new_edges: HashMap<Index, Adjacency<T>> = HashMap::new();
        let mut new_nodes: HashMap<Index, Node<T>> = HashMap::new();

        if let Some(n) = node {
            if !is_directed {
                for i in &edge_adjacency.edges {
                    let mut edge = i.clone();
                    edge.node = n.clone();
                    let other_adjacencies = Adjacency::new(edge);
                    new_edges.insert(i.node.number, other_adjacencies);
                }
            }
            new_edges.insert(n.number, edge_adjacency);
            new_nodes.insert(n.number, n);
        }

        Self {
            nodes: new_nodes,
            adjacency: new_edges,
            is_directed,
        }
    }

    /// Returns the some node index of this [`Graph<T>`].
    pub fn get_some_node_index(&self) -> Option<&Index> {
        self.nodes.keys().next()
    }

    /// Returns nodes with white color of this [`Graph<T>`].
    pub fn get_nodes_with_color(&self) -> HashMap<Index, ColorNode> {
        self.nodes
            .iter()
            .map(|(&index, _)| (index, ColorNode::White))
            .collect::<HashMap<Index, ColorNode>>()
    }

    /// Returns the length of this [`Graph<T>`].
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the is directed field of this [`Graph<T>`].
    pub fn get_is_directed(&self) -> bool {
        self.is_directed
    }

    pub fn add_node(&mut self, node: Node<T>) -> Result<()> {
        if let Entry::Vacant(adj) = self.adjacency.entry(node.number) {
            adj.insert(Adjacency::default());
            self.nodes.insert(node.number, node);
            Ok(())
        } else {
            Err(Box::new(GraphError::new(
                GraphKindError::NodeAlreadyExist,
                "",
            )))
        }
    }

    pub fn add_edge(&mut self, node: &Node<T>, new_edge: &Edge<T>) -> Result<()> {
        if let Some(edges) = self.adjacency.get_mut(&node.number) {
            edges.push(new_edge.clone())?;
        } else {
            return Err(Box::new(GraphError::new(GraphKindError::NodeNotFound, "")));
        }

        if !self.is_directed {
            let duplicate_edge = Edge::new(&node.number, new_edge.weight, &node.value);
            if let Some(edges) = self.adjacency.get_mut(&new_edge.node.number) {
                edges.push(duplicate_edge)?;
            } else {
                return Err(Box::new(GraphError::new(GraphKindError::NodeNotFound, "")));
            }
        }
        Ok(())
    }

    pub fn delete_edge(&mut self, node: &Node<T>, edge_index: &Index) -> Result<PairEdges<T>> {
        let first = if let Some(adjacency) = self.adjacency.get_mut(&node.number) {
            adjacency.delete(*edge_index)?
        } else {
            return Err(Box::new(GraphError::new(GraphKindError::NodeNotFound, "")));
        };

        if self.is_directed {
            Ok((first, None))
        } else {
            let second = if first.is_some()
                && let Some(adjacency) = self.adjacency.get_mut(&first.clone().unwrap().node.number)
            {
                adjacency.delete(node.number)?
            } else {
                return Err(Box::new(GraphError::new(GraphKindError::NodeNotFound, "")));
            };
            Ok((first, second))
        }
    }

    pub fn delete_node(&mut self, node: &Node<T>) -> Result<Adjacency<T>> {
        // Remove edges from other adjacencies
        for (_, adjacency) in self.adjacency.iter_mut() {
            adjacency.edges.retain(|n| n.node.number != node.number);
        }
        self.nodes.remove(&node.number);
        if let Some(adjacency) = self.adjacency.remove(&node.number) {
            return Ok(adjacency);
        }
        Err(Box::new(GraphError::new(GraphKindError::NodeNotFound, "")))
    }

    pub fn write_in_file(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }

    pub fn new_from_file(path: &str) -> Result<Graph<T>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let readed: Graph<T> = serde_json::from_reader(reader)?;
        Ok(readed)
    }

    pub fn to_directed_mut(&mut self) {
        self.is_directed = true;
    }

    pub fn has_edge(&self, from_ind: &Index, to_ind: &Index) -> Result<bool> {
        if let Some(adj) = self.get_adjacency(from_ind) {
            for e in adj {
                if &e.node.number == to_ind {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn to_not_directed(&self) -> Result<Graph<T>> {
        let mut not_dir_graph = Graph {
            is_directed: false,
            ..Default::default()
        };

        if self.len() == 0 {
            return Ok(not_dir_graph);
        }

        for (ind, _) in self {
            if let Some(node) = self.get_node(ind) {
                not_dir_graph.add_node(node.clone())?
            }
        }

        for (ind, adj) in self.iter() {
            for edge in adj {
                not_dir_graph.add_edge(
                    &Node {
                        number: *ind,
                        ..Default::default()
                    },
                    edge,
                )?;
            }
        }

        Ok(not_dir_graph)
    }

    pub fn get_node(&self, index_node: &Index) -> Option<&Node<T>> {
        self.nodes.get(index_node)
    }

    pub fn get_adjacency(&self, index_node: &Index) -> Option<&Adjacency<T>> {
        self.adjacency.get(index_node)
    }

    pub fn get_not_visited_nodes_index(&self, visited: &HashSet<Index>) -> Vec<&Index> {
        self.nodes
            .keys()
            .filter(|n| !visited.contains(*n))
            .collect()
    }

    pub fn create_subgraph(&self, nodes: Vec<Index>, is_directed: bool) -> Result<Self> {
        let mut subgraph: Graph<T> = Graph {
            is_directed,
            ..Default::default()
        };

        for ind in nodes.iter() {
            if let Some(node) = self.get_node(ind) {
                subgraph.add_node(node.clone())?
            }
        }

        for (ind, adj) in self.iter() {
            if subgraph.get_node(ind).is_some() {
                for edge in adj {
                    if subgraph.get_node(&edge.node.number).is_some() {
                        subgraph.add_edge(
                            &Node {
                                number: *ind,
                                ..Default::default()
                            },
                            edge,
                        )?;
                    }
                }
            }
        }

        Ok(subgraph)
    }

    /// Returns the get all edges of this [`Graph<T>`].
    pub fn get_all_edges(&self) -> Vec<(Index, Edge<T>)> {
        let mut edges_list: Vec<(Index, Edge<T>)> = Vec::new();
        for (&ind, adj) in self {
            for edge in adj {
                edges_list.push((ind, edge.clone()));
            }
        }

        edges_list.sort_by(|(_, edge1), (_, edge2)| edge1.weight.cmp(&edge2.weight));
        edges_list
    }

    /// Returns the iter of this [`Graph<T>`].
    pub fn iter(&self) -> GraphIter<'_, T> {
        GraphIter {
            inner: self.adjacency.iter(),
        }
    }

    /// Returns the mut iter of this [`Graph<T>`].
    pub fn iter_mut(&mut self) -> GraphIterMut<'_, T> {
        GraphIterMut {
            inner: self.adjacency.iter_mut(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GraphType {
    Default,
    Tree,
    Forest,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ColorNode {
    White,
    Gray,
    Black,
}

pub struct GraphIter<'a, T>
where
    T: Clone,
{
    inner: hash_map::Iter<'a, Index, Adjacency<T>>,
}

impl<'a, T> Iterator for GraphIter<'a, T>
where
    T: Clone,
{
    type Item = (&'a Index, &'a Adjacency<T>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct GraphIterMut<'a, T>
where
    T: Clone,
{
    inner: hash_map::IterMut<'a, Index, Adjacency<T>>,
}

impl<'a, T> Iterator for GraphIterMut<'a, T>
where
    T: Clone,
{
    type Item = (&'a Index, &'a mut Adjacency<T>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T> IntoIterator for &'a Graph<T>
where
    T: Clone,
{
    type Item = (&'a Index, &'a Adjacency<T>);
    type IntoIter = GraphIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        GraphIter {
            inner: self.adjacency.iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a mut Graph<T>
where
    T: Clone,
{
    type Item = (&'a Index, &'a mut Adjacency<T>);
    type IntoIter = GraphIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        GraphIterMut {
            inner: self.adjacency.iter_mut(),
        }
    }
}
```
== Пример интерфейса в консоли
Примечание: в меню уже добавлены пункты, которые выполняли задания, которые были выданы на практике.

```
Создать ориентированный граф?
false
===========================================================
0. Вывести граф.
1. Добавить вершину.
2. Добавить ребро.
3. Удалить вершину.
4. Удалить ребро.
5. Сохранить в файл.
6. Создать из файла.
7. Вывести полустепень захода данной вершины орграфа. (задание 2)
8. Для каждой вершины орграфа вывести её степень. (задание 3)
9. Построить граф, являющийся пересечением двух заданных. (задание 4)
10. Проверить, является ли граф деревом, или лесом, или не является ни тем, ни другим. (задание 5)
11. Выяснить, является ли граф связным. (задание 6)
12. Найти каркас минимального веса в неориентированном графе (задание 7)
13. Определить, существует ли путь длиной не более L между двумя заданными вершинами графа. (задание 8)
14. Определить, есть ли в графе вершина, минимальные стоимости путей от которой до остальных в сумме не превосходят P. (задание 9)
15. Определить, есть ли в графе вершина, каждая из минимальных стоимостей пути от которой до остальных не превосходит N. (задание 10)
16. Решить задачу на нахождение максимального потока любым алгоритмом. (задание 11)
17 и больше. Выйти
===========================================================
```
