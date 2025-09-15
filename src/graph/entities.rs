use std::{collections::HashMap, fmt::Display};

type Index = u32;
type Weight = u32;

// Edge part

pub struct Edge<T> {
    pub number: Index,
    pub weight: Weight,
    pub value: T,
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
    edges: Vec<Option<Edge<T>>>,
}

impl<T> Default for Adjacency<T> {
    fn default() -> Self {
        Self { edges: Vec::new() }
    }
}

impl<T> Display for Adjacency<T> 
where 
	T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_format = String::from("[\n");
        for edge in &self.edges {
            if let Some(ed) = edge {
                print_format.push_str(format!("{ed},\n").as_str());
            }
        }
		print_format.push_str("]\n");
        write!(f, "{}", print_format)
    }
}

impl<T> Adjacency<T> {
    /// Creates a new [`Adjacency<T>`].
    pub fn new(edge: Edge<T>) -> Self {
        Self {
            edges: vec![Some(edge)],
        }
    }
}

pub struct AdjacencyList<T> {
    edges: HashMap<Index, Adjacency<T>>,
}

impl<T> Default for AdjacencyList<T> {
    fn default() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
}

impl<T> AdjacencyList<T> {
    /// Creates a new [`AdjacencyList<T>`].
    pub fn new(index_edge: Index, edge_adjacency: Adjacency<T>) -> Self {
        let mut new_edges: HashMap<Index, Adjacency<T>> = HashMap::new();
        new_edges.insert(index_edge, edge_adjacency);

        Self { edges: new_edges }
    }
}
