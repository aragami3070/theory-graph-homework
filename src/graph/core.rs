use std::{
    collections::{HashMap, HashSet, hash_map, hash_set},
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    hash::Hash,
    io::{BufReader, BufWriter, Write},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
type Index = u32;
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
            number: 0,
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
    T: Clone + Serialize + DeserializeOwned + Debug,
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

    pub fn get_is_directed(&self) -> bool {
        self.is_directed
    }

    pub fn add_node(&mut self, node: Node<T>) -> Result<()> {
        if !self.adjacency.contains_key(&node.number) {
            self.adjacency.insert(node.number, Adjacency::default());
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

    pub fn to_directed(&mut self) {
        self.is_directed = true
    }
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
