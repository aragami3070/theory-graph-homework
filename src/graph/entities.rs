type Index = u32;
type Weight = u32;

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

impl<T> Edge<T> {
    pub fn new(number: Index, weight: Weight, value: T) -> Self {
        Self {
            number,
            weight,
            value,
        }
    }
}

pub struct Adjacency<T> {
    edges: Vec<Option<Edge<T>>>,
}

impl<T> Adjacency<T> {
    pub fn new(edge: Edge<T>) -> Self {
        Self {
            edges: vec![Some(edge)],
        }
    }
}
