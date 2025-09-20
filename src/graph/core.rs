use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};
type Index = u32;
type Weight = u32;

// Edge part

pub struct Edge<T> {
    pub number: Index,
    pub weight: Weight,
    pub value: T,
}

impl<T> PartialEq for Edge<T> {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
    fn ne(&self, other: &Self) -> bool {
        self.number != other.number
    }
}

impl<T> Eq for Edge<T> {}

impl<T> Hash for Edge<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl<T> Default for Edge<T>
where
    T: Default,
{
    fn default() -> Self {
        Edge::<T> {
            number: 0,
            weight: 0,
            value: T::default(),
        }
    }
}

impl<T> Display for Edge<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  Edge: {{\n    Number: {},\n    Weight: {},\n    Value: {}\n  }}",
            self.number, self.weight, self.value
        )
    }
}

impl<T> Edge<T> {
    /// Creates a new [`Edge<T>`].
    pub fn new(number: Index, weight: Weight, value: T) -> Self {
        Self {
            number,
            weight,
            value,
        }
    }
}

// Adjacency part

pub struct Adjacency<T> {
    edges: HashSet<Edge<T>>,
}

impl<T> Default for Adjacency<T> {
    fn default() -> Self {
        Self {
            edges: HashSet::new(),
        }
    }
}

impl<T> Display for Adjacency<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_format = String::from("[\n");
        for edge in &self.edges {
            print_format.push_str(format!("{edge},\n").as_str());
        }
        print_format.push_str("]");
        write!(f, "{}", print_format)
    }
}

impl<T> Adjacency<T> {
    /// Creates a new [`Adjacency<T>`]
    pub fn new(edge: Edge<T>) -> Self {
        let mut new_edges = HashSet::new();
        new_edges.insert(edge);
        Self { edges: new_edges }
    }

    /// Add in end new [`Edge<T>`]
    pub fn push(&mut self, edge: Edge<T>) {
        self.edges.insert(edge);
    }
}

pub struct AdjacencyList<T> {
    edges: HashMap<Index, Adjacency<T>>,
}

impl<T> Display for AdjacencyList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_format = String::new();
        for adjacency in &self.edges {
            print_format.push_str(format!("{}: {},\n", adjacency.0, adjacency.1).as_str());
        }
        write!(f, "{}", print_format)
    }
}

impl<T> Default for AdjacencyList<T> {
    fn default() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
}

impl<T> AdjacencyList<T> {
    /// Creates a new [`AdjacencyList<T>`]
    pub fn new(index_node: Index, edge_adjacency: Adjacency<T>) -> Self {
        let mut new_edges: HashMap<Index, Adjacency<T>> = HashMap::new();
        new_edges.insert(index_node, edge_adjacency);

        Self { edges: new_edges }
    }

    pub fn add_node(&mut self, index_node: Index) {
        self.edges.insert(index_node, Adjacency::default());
    }
}
