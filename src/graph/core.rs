use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};
type Index = u32;
type Weight = u32;

// Node part

#[derive(Clone)]
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

#[derive(Clone)]
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
    T: Default,
    T: Clone,
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
    T: Display,
    T: Clone,
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

#[derive(Clone)]
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
    T: Display,
    T: Clone,
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

    /// Add in end new [`Edge<T>`]
    fn push(&mut self, edge: Edge<T>) {
        self.edges.insert(edge);
    }

    fn delete(&mut self, edge_index: Index) -> Option<Edge<T>> {
        let rm_edge = self
            .edges
            .iter()
            .find(|e| e.node.number == edge_index)
            .cloned();

        if let Some(rm_e) = rm_edge {
            self.edges.remove(&rm_e);
            Some(rm_e)
        } else {
            None
        }
    }
}

// AdjacencyList part

#[derive(Clone)]
pub struct AdjacencyList<T>
where
    T: Clone,
{
    edges: HashMap<Index, Adjacency<T>>,
    is_directed: bool,
}

impl<T> Display for AdjacencyList<T>
where
    T: Display,
    T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_format = String::new();
        for adjacency in &self.edges {
            print_format.push_str(format!("{}: {},\n", adjacency.0, adjacency.1).as_str());
        }
        write!(f, "{print_format}")
    }
}

impl<T> Default for AdjacencyList<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self {
            edges: HashMap::new(),
            is_directed: false,
        }
    }
}

impl<T> AdjacencyList<T>
where
    T: Clone,
{
    /// Creates a new [`AdjacencyList<T>`]
    pub fn new(index_node: Index, edge_adjacency: Adjacency<T>, is_directed: bool) -> Self {
        let mut new_edges: HashMap<Index, Adjacency<T>> = HashMap::new();
        new_edges.insert(index_node, edge_adjacency);

        Self {
            edges: new_edges,
            is_directed,
        }
    }

    pub fn add_node(&mut self, index_node: Index) {
        self.edges.insert(index_node, Adjacency::default());
    }

    pub fn add_edge(&mut self, node: &Node<T>, new_edge: &Edge<T>) {
        if let Some(edges) = self.edges.get_mut(&node.number) {
            edges.push(new_edge.clone());
        } else {
            self.edges
                .insert(node.number, Adjacency::new(new_edge.clone()));
        }

        if !self.is_directed {
            let duplicate_edge = Edge::new(&node.number, new_edge.weight, &node.value);
            if let Some(edges) = self.edges.get_mut(&new_edge.node.number) {
                edges.push(duplicate_edge);
            } else {
                self.edges
                    .insert(new_edge.node.number, Adjacency::new(duplicate_edge));
            }
        }
    }

    pub fn delete_edge(
        &mut self,
        node: &Node<T>,
        edge_index: &Index,
    ) -> (Option<Edge<T>>, Option<Edge<T>>) {
        let first = if let Some(adjacency) = self.edges.get_mut(&node.number) {
            adjacency.delete(*edge_index)
        } else {
            None
        };

        if self.is_directed {
            (first, None)
        } else {
            let second = if first.is_some()
                && let Some(adjacency) = self.edges.get_mut(&first.clone().unwrap().node.number)
            {
                adjacency.delete(node.number)
            } else {
                None
            };
            (first, second)
        }
    }

    pub fn delete_node(&mut self, node: &Node<T>) -> Option<Adjacency<T>> {
        // Remove edges from other adjacencies
        for (_, adjacency) in self.edges.iter_mut() {
            adjacency.edges.retain(|n| n.node.number != node.number);
        }
        self.edges.remove(&node.number)
    }
}
